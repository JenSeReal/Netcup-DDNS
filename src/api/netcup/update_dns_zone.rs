use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

use super::{info_dns_zone, Action, ApiSessionId, SessionCredentials};

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
    session: &SessionCredentials<ApiSessionId>,
    domain_name: &str,
    dns_zone: &info_dns_zone::ResponseData,
  ) -> Self {
    Self {
      action: Action::UpdateDnsZone,
      param: Param {
        domain_name: domain_name.to_string(),
        customer_number: session.customer_number,
        api_key: session.api_key.to_string(),
        api_session_id: session.api_session_id().to_string(),
        dns_zone: dns_zone.clone(),
      },
    }
  }
}
