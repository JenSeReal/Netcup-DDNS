mod api;
mod cli;
mod errors;
mod serialization;

use std::{env, time::Duration};

use api::{
  ip::current_ip,
  netcup::{info_dns_zone, login},
};
use dotenv::dotenv;
use error_stack::{IntoReport, Report, ResultExt};
use log::{debug, info, warn};
use structopt::StructOpt;
use tokio::time::sleep;

use crate::{
  api::netcup::{logout, update_dns_zone, Response, Status},
  cli::Cli,
  errors::Errors,
};

#[tokio::main]
async fn main() -> error_stack::Result<(), Errors> {
  env::set_var("RUST_LOG", "debug");
  dotenv()
    .into_report()
    .change_context(Errors::LoadingEnvFile)?;
  env_logger::init();

  let cli = Cli::from_args();
  let client = reqwest::Client::new();

  let login_response: login::Response = api::netcup::request(
    cli.api_url(),
    &client,
    &login::Request::new(cli.customer_number(), cli.api_key(), cli.api_password()),
  )
  .await
  .change_context(Errors::Login)?;

  info!("Login successful");

  let api_session_id = login_response
    .api_session_id()
    .ok_or_else(|| Report::new(Errors::RetrieveAPISesionId))?;

  let ip = current_ip(cli.ip_url(), &client).await?;

  info!("Got IP {:?}", ip);

  for domain in cli.domains() {
    debug!("{domain:#?}");

    let dns_zone_info_response: info_dns_zone::Response = api::netcup::request(
      cli.api_url(),
      &client,
      &info_dns_zone::Request::new(
        domain.domain(),
        cli.customer_number(),
        cli.api_key(),
        &api_session_id,
      ),
    )
    .await
    .change_context(Errors::DNSZoneNotFound(domain.domain().to_string()))?;

    if dns_zone_info_response.status() != Status::Success {
      Err(Errors::DNSZoneNotFound(domain.domain().to_string()))?
    }

    if let Some(mut response_data) = dns_zone_info_response.response_data().cloned() {
      let current_ttl = response_data.ttl();
      if current_ttl > 300 {
        warn!("TTL is {current_ttl} and should be 300");
        if let Some(ttl) = cli.ttl() {
          response_data.ttl_mut(ttl);
          let _update_dns_zone_response: update_dns_zone::Response = api::netcup::request(
            cli.api_url(),
            &client,
            &update_dns_zone::Request::new(
              domain.domain(),
              cli.customer_number(),
              cli.api_key(),
              &api_session_id,
              &response_data,
            ),
          )
          .await
          .change_context(Errors::UpdateDNSZone(domain.domain().to_string()))?;
        }
      }
    }
  }

  sleep(Duration::from_secs(2)).await;

  api::netcup::request::<logout::Request, logout::Response>(
    cli.api_url(),
    &client,
    &logout::Request::new(cli.customer_number(), cli.api_key(), &api_session_id),
  )
  .await
  .change_context(Errors::Logout)?;

  info!("Logout successful");

  Ok(())
}
