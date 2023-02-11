use std::{net::IpAddr, str::FromStr};

use error_stack::{IntoReport, ResultExt};
use log::{debug, info};
use reqwest::Client;

use crate::errors::Errors;

pub(crate) async fn current_ip(url: &str, client: &Client) -> error_stack::Result<IpAddr, Errors> {
  let res = client
    .get(url)
    .send()
    .await
    .into_report()
    .change_context(Errors::SendRequest)
    .attach_printable(format!("Could not send request {:?}", url))?;

  debug!("Recieved response: {:?}", res);

  let ip = res
    .text()
    .await
    .map(|s| IpAddr::from_str(&s))
    .into_report()
    .change_context(Errors::SerializeResponse)
    .attach_printable("Could not serialize ip address.")?
    .into_report()
    .change_context(Errors::SerializeIp)?;

  info!("Found ip address: {ip:#?}");

  Ok(ip)
}
