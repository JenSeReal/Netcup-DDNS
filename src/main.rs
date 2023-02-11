mod api;
mod cli;
mod errors;
mod serialization;

use std::{env, time::Duration};

use api::{
  ip::current_ip,
  netcup::{info_dns_zone, login::LoginResponse},
};
use dotenv::dotenv;
use error_stack::{IntoReport, Report, ResultExt};
use log::{debug, info, warn};
use structopt::StructOpt;
use tokio::time::sleep;

use crate::{
  api::netcup::{
    login::LoginRequest,
    logout::{LogoutRequest, LogoutResponse},
    Response, Status,
  },
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

  let login_response: LoginResponse = api::netcup::request(
    cli.api_url(),
    &client,
    &LoginRequest::new(cli.customer_number(), cli.api_key(), cli.api_password()),
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

    if let Some(ttl) = dns_zone_info_response.ttl() {
      if ttl > 300 {
        warn!("TTL is {ttl} and should be 300");
      }
    }
  }

  sleep(Duration::from_secs(2)).await;

  api::netcup::request::<LogoutRequest, LogoutResponse>(
    cli.api_url(),
    &client,
    &LogoutRequest::new(cli.customer_number(), cli.api_key(), &api_session_id),
  )
  .await
  .change_context(Errors::Logout)?;

  info!("Logout successful");

  Ok(())
}
