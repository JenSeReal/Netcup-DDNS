use std::{env, time::Duration};

use cli::Cli;
use dotenv::dotenv;
use errors::Errors;
use log::{debug, error, info, warn};
use structopt::StructOpt;
use tokio::time::sleep;

use crate::api::netcup::{self, models::DnsRecord};

mod api;
mod cli;
mod errors;
mod serialization;

#[tokio::main]
async fn main() -> error_stack::Result<(), Errors> {
  env::set_var("RUST_LOG", "debug");
  dotenv().ok();
  env_logger::init();

  let cli = Cli::from_args();
  let client = netcup::Client::new(&cli);

  let client = client.login().await?;

  let ips = api::ip::external().await;
  ips.iter().for_each(|ip| info!("Got IP {ip:?}"));

  for domain_zone in cli.domains() {
    info!("Looking at domain-zone {:#?}", domain_zone);

    let info_dns_zone_response = match client.info_dns_zone(domain_zone.domain()).await {
      Ok(info_dns_zone_response) => info_dns_zone_response,
      Err(e) => {
        error!("{e}");
        continue;
      }
    };

    if let Some(mut response_data) = info_dns_zone_response.response_data().cloned() {
      let current_ttl = response_data.ttl();
      if current_ttl > 300 {
        warn!("TTL is {current_ttl} and should be 300");
        if let Some(ttl) = cli.ttl() {
          response_data.ttl_mut(ttl);
          info!("Changing TTL to {}", ttl);
          let update_dns_zone_response = match client
            .update_dns_zone(domain_zone.domain(), response_data)
            .await
          {
            Ok(update_dns_zone_response) => update_dns_zone_response,
            Err(e) => {
              error!("{e}");
              continue;
            }
          };
          if update_dns_zone_response.status_code() != netcup::StatusCode::Success {
            error!(
              "{}",
              Errors::UpdateDNSZone(domain_zone.domain().to_string())
            );
            continue;
          }
          info!("Updated dns zone!");
        }
      }
    }

    info!("Getting all dns records");
    let info_dns_records_response = match client.info_dns_records(domain_zone.domain()).await {
      Ok(info_dns_records_response) => info_dns_records_response,
      Err(e) => {
        error!("{e}");
        continue;
      }
    };

    let dns_records = match info_dns_records_response
      .response_data()
      .map(|data| data.dns_records())
    {
      Some(dns_records) => dns_records,
      None => {
        error!("No info about dns records");
        continue;
      }
    };

    for sub_domain in domain_zone.sub_domains() {
      info!("Looking at {sub_domain:#?} subdomain");

      let found_records = dns_records
        .iter()
        .filter(|record| record.host_name() == sub_domain)
        .filter(|record| !matches!(record.record_type(), netcup::models::RecordType::Other(_)))
        .collect::<Vec<_>>();

      debug!("Found records: {:#?}", found_records);

      match found_records.len() {
        0 => {
          info!("No DNS record found for {sub_domain:#?} subdomain.. creating one..");

          let dns_records = ips.iter().fold(vec![], |mut dns_records, destination| {
            dns_records.push(DnsRecord::new(sub_domain, *destination));
            dns_records
          });

          let update_dns_records_response = match client
            .update_dns_records(domain_zone.domain(), dns_records)
            .await
          {
            Ok(update_dns_records_response) => update_dns_records_response,
            Err(e) => {
              error!("{e}");
              continue;
            }
          };
        }
        1 => {
          info!("One DNS record found for {sub_domain:#?} subdomain.. updating..");
        }
        _ => {
          error!("Too many DNS records found for {sub_domain:#?} subdomain.. please specify");
        }
      }
    }
  }

  sleep(Duration::from_secs(2)).await;

  client.logout().await?;
  Ok(())
}
