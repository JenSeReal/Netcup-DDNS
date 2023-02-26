use error_stack::{IntoReport, Report, ResultExt};
use log::{debug, error, info};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use std::fmt::{Debug, Display};

use crate::{
  errors::Errors,
  serialization::{empty_string_as_none, opt_string_or_struct},
};

pub mod info_dns_records;
pub mod info_dns_zone;
pub mod login;
pub mod logout;
pub mod update_dns_zone;

pub async fn request<Rq, Rs>(
  url: &str,
  client: &Client,
  request: &Rq,
) -> error_stack::Result<Response<Rs>, Errors>
where
  Rq: Serialize + Sized + Debug,
  Rs: DeserializeOwned + Sized + Debug,
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

    let response_object = serde_json::from_str::<Response<Rs>>(&body)
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
  UpdateDnsZone,
  InfoDnsRecords,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionCredentials {
  #[serde(rename = "customernumber")]
  customer_number: u32,
  #[serde(rename = "apikey")]
  api_key: String,
  #[serde(rename = "apisessionid")]
  api_session_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response<T>
where
  T: DeserializeOwned,
{
  #[serde(rename = "serverrequestid")]
  server_request_id: String,
  #[serde(rename = "clientrequestid", deserialize_with = "empty_string_as_none")]
  client_request_id: Option<String>,
  #[serde(deserialize_with = "empty_string_as_none")]
  action: Option<Action>,
  status: Status,
  #[serde(rename = "statuscode")]
  status_code: StatusCode,
  #[serde(rename = "shortmessage")]
  short_message: String,
  #[serde(rename = "longmessage")]
  long_message: Option<String>,
  #[serde(rename = "responsedata", deserialize_with = "opt_string_or_struct")]
  response_data: Option<T>,
}

impl<T> Response<T>
where
  T: DeserializeOwned,
{
  pub fn status_code(&self) -> StatusCode {
    self.status_code
  }

  pub fn status(&self) -> Status {
    self.status
  }

  pub fn response_data(&self) -> Option<&T> {
    self.response_data.as_ref()
  }
}
