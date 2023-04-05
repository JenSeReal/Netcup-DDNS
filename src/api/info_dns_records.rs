use std::{net::IpAddr, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_with::DeserializeFromStr;

use crate::errors::Errors;

use super::{Action, ApiSessionId, SessionCredentials};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
  action: Action,
  param: Param,
}

impl Request {
  pub fn new(session: &SessionCredentials<ApiSessionId>, domain_name: &str) -> Self {
    Self {
      action: Action::InfoDnsRecords,
      param: Param {
        domain_name: domain_name.to_string(),
        customer_number: session.customer_number,
        api_key: session.api_key.to_string(),
        api_session_id: session.api_session_id().to_string(),
      },
    }
  }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
  #[serde(rename = "domainname")]
  domain_name: String,
  #[serde(rename = "customernumber")]
  customer_number: u32,
  #[serde(rename = "apikey")]
  api_key: String,
  #[serde(rename = "apisessionid")]
  api_session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

pub struct ResponseData {
  #[serde(rename = "dnsrecords")]
  dns_records: Vec<DnsRecord>,
}

impl ResponseData {
  pub fn dns_records(&self) -> &Vec<DnsRecord> {
    &self.dns_records
  }
}

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
  id: String,
  #[serde(rename = "hostname")]
  host_name: String,
  #[serde(rename = "type")]
  _type: RecordType,
  priority: String,
  destination: IpType,
  #[serde(rename = "deleterecord")]
  delete_record: bool,
  state: String,
}

impl DnsRecord {
  pub fn host_name(&self) -> &str {
    &self.host_name
  }

  pub fn record_type(&self) -> &RecordType {
    &self._type
  }

  pub fn destination(&self) -> &IpType {
    &self.destination
  }
}
