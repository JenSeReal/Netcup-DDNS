use error_stack::{IntoReport, Report, ResultExt};
use log::{debug, error, info};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use std::fmt::Debug;

use crate::{cli::Cli, errors::Errors};
use models::{Request, Response};

use self::models::{NoApiSessionId, SessionCredentials};

pub mod info_dns_records;
pub mod info_dns_zone;
pub mod login;
pub mod logout;
pub mod models;
pub mod update_dns_records;
pub mod update_dns_zone;

pub struct Client<T> {
  client: reqwest::Client,
  api_url: String,
  session_credentials: SessionCredentials<T>,
}

impl Client<NoApiSessionId> {
  pub fn new(cli: &Cli) -> Self {
    Self {
      client: reqwest::Client::new(),
      api_url: cli.api_url().into(),
      session_credentials: SessionCredentials::new(
        cli.customer_number(),
        cli.api_key(),
        cli.api_password(),
      ),
    }
  }
}

async fn request<Rq, Rs>(
  url: &str,
  client: &reqwest::Client,
  request: &Request<Rq>,
) -> error_stack::Result<Response<Rs>, Errors>
where
  Rq: Serialize + Sized + Debug,
  Rs: DeserializeOwned + Sized + Debug,
{
  info!("Performing request: {:#?} {request:#?}", request.action());
  let resonse = client
    .post(url)
    .json(&request)
    .send()
    .await
    .into_report()
    .change_context(Errors::SendRequest)
    .attach_printable(format!("Could not send request {:#?}", request))?;

  debug!("Recieved response: {:#?}", resonse);

  if !resonse.status().is_success() {
    error!(
      "Http status code {} while performing the request",
      resonse.status()
    );
    return Err(Report::new(Errors::SendRequest));
  }

  let body = resonse
    .text()
    .await
    .into_report()
    .change_context(Errors::SerializeResponse)?;

  debug!("Recieved response body: {:#?}", body);

  let response_object = serde_json::from_str::<Response<Rs>>(&body)
    .into_report()
    .change_context(Errors::SerializeResponse)?;

  debug!("Serialized response body: {:#?}", response_object);

  match response_object.status_code() {
    StatusCode::Success => {
      info!("Request {:#?} was successful.", request.action());
      Ok(response_object)
    }
    StatusCode::Error => {
      error!(
        "Request {:#?} wasn't successful. Maybe the API key is not valid",
        request.action()
      );
      Err(Errors::SendRequest.into())
    }
    StatusCode::ValidationError => {
      error!(
        "Request {:#?} wasn't successful. Maybe too many requests in one hour.",
        request.action()
      );
      Err(Errors::ValidationError.into())
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Action {
  Login,
  Logout,
  InfoDnsZone,
  UpdateDnsZone,
  InfoDnsRecords,
  UpdateDnsRecords,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
  Error,
  Started,
  Pending,
  Warning,
  Success,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StatusCode {
  Success = 2000,
  Error = 4001,
  ValidationError = 4013,
}
