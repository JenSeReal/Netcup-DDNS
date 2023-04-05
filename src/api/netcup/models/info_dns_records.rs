use serde::{Deserialize, Serialize};

use super::{ApiSessionId, DnsRecord, SessionCredentials};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Params {
  #[serde(rename = "domainname")]
  domain_name: String,
  #[serde(rename = "customernumber")]
  customer_number: u32,
  #[serde(rename = "apikey")]
  api_key: String,
  #[serde(rename = "apisessionid")]
  api_session_id: String,
}

impl Params {
  pub fn new(
    domain_name: impl Into<String>,
    session_credentials: &SessionCredentials<ApiSessionId>,
  ) -> Self {
    Self {
      domain_name: domain_name.into(),
      customer_number: session_credentials.customer_number(),
      api_key: session_credentials.api_key().into(),
      api_session_id: session_credentials.api_session_id().into(),
    }
  }
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
