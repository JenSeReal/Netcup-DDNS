use std::{marker::PhantomData, net::IpAddr, str::FromStr};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::DeserializeFromStr;

use crate::{
  errors::Errors,
  serialization::{empty_string_as_none, opt_string_or_struct},
};

use super::{Action, Status, StatusCode};

pub mod info_dns_records;
pub mod info_dns_zone;
pub mod login;
pub mod logout;
pub mod update_dns_records;
pub mod update_dns_zone;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request<T> {
  action: Action,
  param: T,
}

impl<T> Request<T> {
  pub fn new(action: Action, param: T) -> Self {
    Self { action, param }
  }

  pub fn action(&self) -> &Action {
    &self.action
  }
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

  // pub fn status(&self) -> Status {
  //   self.status
  // }

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

  pub fn api_session_id(
    self,
    api_session_id: impl Into<String>,
  ) -> SessionCredentials<ApiSessionId> {
    SessionCredentials {
      customer_number: self.customer_number,
      api_key: self.api_key,
      api_password: self.api_password,
      api_session_id: Some(api_session_id.into()),
      has_api_session_id: PhantomData,
    }
  }
}

impl SessionCredentials<ApiSessionId> {
  pub fn api_session_id(&self) -> &str {
    self.api_session_id.as_ref().unwrap()
  }
}

impl<T> SessionCredentials<T> {
  pub fn customer_number(&self) -> u32 {
    self.customer_number
  }

  pub fn api_key(&self) -> &str {
    &self.api_key
  }

  pub fn api_password(&self) -> &str {
    &self.api_password
  }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, PartialEq, Eq, DeserializeFromStr)]
pub enum RecordType {
  A,
  AAAA,
  Other(String),
}

impl FromStr for RecordType {
  type Err = Errors;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "A" => Ok(Self::A),
      "AAAA" => Ok(Self::AAAA),
      _ => Ok(Self::Other(s.to_string())),
    }
  }
}

impl From<IpAddr> for RecordType {
  fn from(value: IpAddr) -> Self {
    match value {
      IpAddr::V4(_) => Self::A,
      IpAddr::V6(_) => Self::AAAA,
    }
  }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, DeserializeFromStr)]
#[serde(untagged)]
pub enum IpType {
  Ip(IpAddr),
  Other(String),
}

impl FromStr for IpType {
  type Err = Errors;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.parse::<IpAddr>() {
      Ok(ip) => Ok(Self::Ip(ip)),
      Err(_) => Ok(Self::Other(s.to_string())),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DnsRecord {
  id: Option<String>,
  #[serde(rename = "hostname")]
  host_name: String,
  #[serde(rename = "type")]
  _type: RecordType,
  priority: Option<String>,
  destination: IpType,
  #[serde(rename = "deleterecord")]
  delete_record: Option<bool>,
  state: Option<String>,
}

impl DnsRecord {
  pub fn new(host_name: impl Into<String>, destination: IpAddr) -> Self {
    Self {
      id: None,
      host_name: host_name.into(),
      _type: destination.into(),
      priority: None,
      destination: IpType::Ip(destination),
      delete_record: None,
      state: None,
    }
  }

  pub fn host_name(&self) -> &str {
    &self.host_name
  }

  pub fn record_type(&self) -> &RecordType {
    &self._type
  }

  // pub fn destination(&self) -> &IpType {
  //   &self.destination
  // }
}
