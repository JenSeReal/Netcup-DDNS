use serde::{Deserialize, Serialize};

use super::{ApiSessionId, DnsRecord, SessionCredentials};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseData {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

struct DnsRecordSet {
  #[serde(rename = "dnsrecords")]
  dns_records: Vec<DnsRecord>,
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
  #[serde(rename = "dnsrecordset")]
  dns_record_set: DnsRecordSet,
}

impl Params {
  pub fn new(
    domain_name: impl Into<String>,
    session_credentials: &SessionCredentials<ApiSessionId>,
    dns_records: Vec<DnsRecord>,
  ) -> Self {
    Self {
      domain_name: domain_name.into(),
      customer_number: session_credentials.customer_number(),
      api_key: session_credentials.api_key().into(),
      api_session_id: session_credentials.api_session_id().into(),
      dns_record_set: DnsRecordSet { dns_records },
    }
  }
}
