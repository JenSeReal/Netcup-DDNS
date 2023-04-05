mod api;
mod cli;
mod errors;
mod serialization;

use std::{env, net::IpAddr, time::Duration};

use api::netcup::{self, info_dns_zone, login};
use dotenv::dotenv;
use error_stack::{IntoReport, Report, ResultExt};
use log::{debug, error, info, warn};
use structopt::StructOpt;
use tokio::time::sleep;

use crate::{
  api::netcup::{
    info_dns_records::{self, RecordType},
    logout, update_dns_zone, Response, SessionCredentials, Status, StatusCode,
  },
  cli::Cli,
  errors::Errors,
};

#[tokio::main]
async fn main() -> error_stack::Result<(), Errors> {
  env::set_var("RUST_LOG", "info");
  dotenv()
    .into_report()
    .change_context(Errors::LoadingEnvFile)?;
  env_logger::init();

  let cli = Cli::from_args();
  let client = reqwest::Client::new();
  let session_credentials =
    SessionCredentials::new(cli.customer_number(), cli.api_key(), cli.api_password());

  let login_response: netcup::Response<login::ResponseData> = api::netcup::request(
    cli.api_url(),
    &client,
    &login::Request::new(&session_credentials),
  )
  .await
  .change_context(Errors::Login)?;

  info!("Login successful");

  let api_session_id = login_response
    .response_data()
    .and_then(|data| Some(data.app_session_id()))
    .ok_or_else(|| Report::new(Errors::RetrieveAPISesionId))?;

  let session_credentials = session_credentials.api_session_id(&api_session_id);

  let (ip4, ip6) = tokio::join!(public_ip::addr_v4(), public_ip::addr_v6());
  let ips = [
    ip4.and_then(|ip| Some(IpAddr::V4(ip))),
    ip6.and_then(|ip| Some(IpAddr::V6(ip))),
  ];

  ips.iter().flatten().for_each(|ip| info!("Got IP {ip:?}"));

  for domain in cli.domains() {
    info!("Looking at domain {:#?}", domain);
    let dns_zone_info_response: netcup::Response<info_dns_zone::ResponseData> =
      api::netcup::request(
        cli.api_url(),
        &client,
        &info_dns_zone::Request::new(&session_credentials, domain.domain()),
      )
      .await
      .change_context(Errors::DNSZoneNotFound(domain.domain().to_string()))?;

    if dns_zone_info_response.status() != Status::Success {
      error!("{}", Errors::DNSZoneNotFound(domain.domain().to_string()));
      continue;
    }

    if let Some(mut response_data) = dns_zone_info_response.response_data().cloned() {
      let current_ttl = response_data.ttl();
      if current_ttl > 300 {
        warn!("TTL is {current_ttl} and should be 300");
        if let Some(ttl) = cli.ttl() {
          response_data.ttl_mut(ttl);
          info!("Changing TTL to {}", ttl);
          if let Ok(update_dns_zone_response) = api::netcup::request::<
            update_dns_zone::Request,
            Response<update_dns_zone::ResponseData>,
          >(
            cli.api_url(),
            &client,
            &update_dns_zone::Request::new(&session_credentials, domain.domain(), &response_data),
          )
          .await
          {
            info!("Updated dns zone!");
            debug!("{:#?}", update_dns_zone_response);
            if update_dns_zone_response.status_code() != StatusCode::Success {
              error!("{}", Errors::UpdateDNSZone(domain.domain().to_string()));
            }
          } else {
            error!("{}", Errors::UpdateDNSZone(domain.domain().to_string()));
          }
        }
      }
    }

    info!("Getting all dns records");
    let info_dns_record: netcup::Response<info_dns_records::ResponseData> = netcup::request(
      cli.api_url(),
      &client,
      &info_dns_records::Request::new(&session_credentials, domain.domain()),
    )
    .await?;

    for sub in domain.sub_domains().iter() {
      info!("Looking at {sub:#?} subdomain");

      if let Some(dns_records) = info_dns_record
        .response_data()
        .and_then(|data| Some(data.dns_records()))
      {
        let found_records = dns_records
          .iter()
          .filter(|record| record.host_name() == sub)
          .filter(|record| !matches!(record.record_type(), RecordType::Other(_)))
          .collect::<Vec<_>>();

        debug!("Found records: {:#?}", found_records);

        match found_records.len() {
          0 => {
            info!("No DNS record found for {sub:#?} subdomain.. creating one..");

            for ip in ips.iter().flatten() {
              info!("The new IP is: {ip}");

              match ip {
                IpAddr::V4(ip) => {}
                IpAddr::V6(ip) => {}
              }
            }
          }
          1 => {
            info!("One DNS record found for {sub:#?} subdomain.. updating..");
          }
          _ => {
            error!("Too many DNS record found for {sub:#?} subdomain.. please specify");
          }
        }
      }
    }
  }

  sleep(Duration::from_secs(2)).await;

  api::netcup::request::<logout::Request, Response<logout::ResponseData>>(
    cli.api_url(),
    &client,
    &logout::Request::new(&session_credentials),
  )
  .await
  .change_context(Errors::Logout)?;

  info!("Logout successful");

  Ok(())
}
