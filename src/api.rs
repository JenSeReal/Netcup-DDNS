pub mod login;
pub mod logout;

use std::fmt::{Debug, Display};

use error_stack::{IntoReport, Report, ResultExt};
use log::{debug, error, info};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::errors::Errors;

pub const URL: &str = "https://ccp.netcup.net/run/webservice/servers/endpoint.php?JSON";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Actions {
  Login,
  Logout,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
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
  ValidationError = 4013,
}

pub trait Response {
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

pub async fn request<Rq, Rs>(client: &Client, request: &Rq) -> error_stack::Result<Rs, Errors>
where
  Rq: Serialize + Sized + Debug,
  Rs: DeserializeOwned + Sized + Response + Debug,
{
  let resonse = client
    .post(URL)
    .json(&request)
    .send()
    .await
    .report()
    .change_context(Errors::SendRequest)
    .attach_printable(format!("Could not send request {:?}", request))?;

  debug!("Recieved response: {:?}", resonse);

  if resonse.status().is_success() {
    let body = resonse
      .text()
      .await
      .report()
      .change_context(Errors::SerializeResponse)?;

    debug!("Recieved response body: {:?}", body);

    let response_object = serde_json::from_str::<Rs>(&body)
      .report()
      .change_context(Errors::SerializeResponse)?;

    debug!("Serialized responde body: {:?}", response_object);

    return match response_object.status_code() {
      StatusCode::Success => {
        info!("Request was successful.");
        Ok(response_object)
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
