use error_stack::{IntoReport, Report, ResultExt};
use log::{debug, error, info};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use std::{fmt::Debug, marker::PhantomData};

use crate::{
  errors::Errors,
  serialization::{empty_string_as_none, opt_string_or_struct},
};

pub mod info_dns_records;
pub mod info_dns_zone;
pub mod login;
pub mod logout;
pub mod update_dns_records;
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
    .attach_printable(format!("Could not send request {:#?}", request))?;

  debug!("Recieved response: {:#?}", resonse);

  if resonse.status().is_success() {
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

    return match response_object.status_code() {
      StatusCode::Success => {
        info!("Request was successful.");
        Ok(response_object)
      }
      StatusCode::Error => {
        error!("Request wasn't successful. Maybe the API key is not valid");
        Err(Errors::SendRequest.into())
      }
      StatusCode::ValidationError => {
        error!("Request wasn't successful. Maybe too many requests in one hour.");
        Err(Errors::ValidationError.into())
      }
    };
  }

  error!(
    "Http status code {} while performing the request",
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request<T> {
  action: Action,
  param: T,
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

pub struct NoApiSessionId;
pub struct ApiSessionId;

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionCredentials<T> {
  #[serde(rename = "customernumber")]
  customer_number: u32,
  #[serde(rename = "apikey")]
  api_key: String,
  #[serde(rename = "apipassword")]
  api_password: String,
  #[serde(rename = "apisessionid")]
  api_session_id: Option<String>,
  #[serde(skip)]
  has_api_session_id: PhantomData<T>,
}

impl SessionCredentials<NoApiSessionId> {
  pub fn new(customer_number: u32, api_key: &str, api_password: &str) -> Self {
    Self {
      customer_number,
      api_key: api_key.to_string(),
      api_password: api_password.to_string(),
      api_session_id: None,
      has_api_session_id: PhantomData,
    }
  }

  pub fn api_session_id(self, api_session_id: &impl ToString) -> SessionCredentials<ApiSessionId> {
    SessionCredentials {
      customer_number: self.customer_number,
      api_key: self.api_key,
      api_password: self.api_password,
      api_session_id: Some(api_session_id.to_string()),
      has_api_session_id: PhantomData,
    }
  }
}

impl SessionCredentials<ApiSessionId> {
  pub fn api_session_id(&self) -> &str {
    self.api_session_id.as_ref().unwrap()
  }
}
