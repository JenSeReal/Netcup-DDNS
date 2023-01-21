mod api;
mod cli;
mod errors;
mod serialization;

use std::{env, time::Duration};

use api::{ip::current_ip, netcup::login::LoginResponse};
use dotenv::dotenv;
use error_stack::{IntoReport, Report, ResultExt};
use log::info;
use structopt::StructOpt;
use tokio::time::sleep;

use crate::{
  api::netcup::{
    login::LoginRequest,
    logout::{LogoutRequest, LogoutResponse},
  },
  cli::Cli,
  errors::Errors,
};

#[tokio::main]
async fn main() -> error_stack::Result<(), Errors> {
  env::set_var("RUST_LOG", "debug");
  dotenv().report().change_context(Errors::LoadingEnvFile)?;
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

  let _ip = current_ip(cli.ip_url(), &client).await?;

  for domain in cli.domains() {
    dbg!(domain);
  }

  sleep(Duration::from_secs(2)).await;

  let api_session_id = login_response
    .api_session_id()
    .ok_or_else(|| Report::new(Errors::RetrieveAPISesionId))?;

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
