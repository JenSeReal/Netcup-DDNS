use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

use crate::{
  api::netcup,
  serialization::{empty_string_as_none, opt_string_or_struct},
};

use super::{info_dns_zone, Action};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ResponseData {
  name: String,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  ttl: u32,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  serial: u32,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  refresh: u32,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  retry: u32,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  expire: u32,
  #[serde(rename = "dnssecstatus")]
  dns_sec_status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Response {
  #[serde(rename = "serverrequestid")]
  server_request_id: String,
  #[serde(rename = "clientrequestid", deserialize_with = "empty_string_as_none")]
  client_request_id: Option<String>,
  #[serde(deserialize_with = "empty_string_as_none")]
  action: Option<netcup::Action>,
  status: netcup::Status,
  #[serde(rename = "statuscode")]
  status_code: netcup::StatusCode,
  #[serde(rename = "shortmessage")]
  short_message: String,
  #[serde(rename = "longmessage")]
  long_message: Option<String>,
  #[serde(rename = "responsedata", deserialize_with = "opt_string_or_struct")]
  response_data: Option<ResponseData>,
}

impl netcup::Response for Response {
  fn status_code(&self) -> netcup::StatusCode {
    self.status_code
  }

  fn status(&self) -> netcup::Status {
    self.status
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
  action: Action,
  param: Param,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
  #[serde(rename = "domainname")]
  domain_name: String,
  #[serde(rename = "customernumber")]
  customer_number: u32,
  #[serde(rename = "apikey")]
  api_key: String,
  #[serde(rename = "apisessionid")]
  api_session_id: String,
  #[serde(rename = "dnszone")]
  dns_zone: info_dns_zone::ResponseData,
}

impl Request {
  pub fn new(
    domain_name: &str,
    customer_number: u32,
    api_key: &str,
    api_session_id: &str,
    dns_zone: &info_dns_zone::ResponseData,
  ) -> Self {
    Self {
      action: Action::UpdateDnsZone,
      param: Param {
        domain_name: domain_name.to_string(),
        customer_number,
        api_key: api_key.to_string(),
        api_session_id: api_session_id.to_string(),
        dns_zone: dns_zone.clone(),
      },
    }
  }
}
