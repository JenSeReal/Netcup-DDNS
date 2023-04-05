use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

use super::{info_dns_zone, ApiSessionId, SessionCredentials};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseData {
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
pub struct Params {
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

impl Params {
  pub fn new(
    domain_name: impl Into<String>,
    session_credentials: &SessionCredentials<ApiSessionId>,
    dns_zone: info_dns_zone::ResponseData,
  ) -> Self {
    Self {
      domain_name: domain_name.into(),
      customer_number: session_credentials.customer_number,
      api_key: session_credentials.api_key().into(),
      api_session_id: session_credentials.api_session_id().into(),
      dns_zone,
    }
  }
}
