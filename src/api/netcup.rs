use error_stack::{IntoReport, Report, ResultExt};
use log::{debug, error, info};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use std::fmt::{Debug, Display};

use crate::errors::Errors;

pub mod info_dns_zone;
pub mod login;
pub mod logout;
pub mod update_dns_zone;

pub async fn request<Rq, Rs>(
  url: &str,
  client: &Client,
  request: &Rq,
) -> error_stack::Result<Rs, Errors>
where
  Rq: Serialize + Sized + Debug,
  Rs: DeserializeOwned + Sized + Response + Debug,
{
  let resonse = client
    .post(url)
    .json(&request)
    .send()
    .await
    .into_report()
    .change_context(Errors::SendRequest)
    .attach_printable(format!("Could not send request {:?}", request))?;

  debug!("Recieved response: {:?}", resonse);

  if resonse.status().is_success() {
    let body = resonse
      .text()
      .await
      .into_report()
      .change_context(Errors::SerializeResponse)?;

    debug!("Recieved response body: {:?}", body);

    let response_object = serde_json::from_str::<Rs>(&body)
      .into_report()
      .change_context(Errors::SerializeResponse)?;

    debug!("Serialized responde body: {:?}", response_object);

    return match response_object.status_code() {
      StatusCode::Success => {
        info!("Request was successful.");
        Ok(response_object)
      }
      StatusCode::Error => {
        info!("Request wasn't successful. Maybe the API key is not valid");
        Err(Errors::SendRequest.into())
      }
      StatusCode::ValidationError => {
        info!("Request wasn't successful. Maybe too many requests in one hour.");
        Err(Errors::ValidationError.into())
      }
    };
  }

  error!(
    "Http statuc code {} while performing the request",
    resonse.status()
  );
  Err(Report::new(Errors::SendRequest))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Action {
  Login,
  Logout,
  InfoDnsZone,
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

impl Display for Status {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Status::Error => todo!(),
      Status::Started => todo!(),
      Status::Pending => todo!(),
      Status::Warning => todo!(),
      Status::Success => todo!(),
    }
  }
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum StatusCode {
  Success = 2000,
  Error = 4001,
  ValidationError = 4013,
}

pub trait Response {
  fn status(&self) -> Status;
  fn status_code(&self) -> StatusCode;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseData {
  #[serde(rename = "apisessionid")]
  app_session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
  #[serde(rename = "customernumber")]
  customer_number: u32,
  #[serde(rename = "apikey")]
  api_key: String,
  #[serde(rename = "apipassword")]
  api_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionCredentials {
  #[serde(rename = "customernumber")]
  customer_number: u32,
  #[serde(rename = "apikey")]
  api_key: String,
  #[serde(rename = "apisessionid")]
  api_session_id: String,
}
