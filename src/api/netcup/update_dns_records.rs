use serde::{Deserialize, Serialize};

use super::{info_dns_records::DnsRecord, Action, ApiSessionId, Request, SessionCredentials};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ResponseData {}

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
  #[serde(rename = "dnsrecordset")]
  dns_record_set: Vec<DnsRecord>,
}

impl Request<Param> {
  pub fn new(
    session: &SessionCredentials<ApiSessionId>,
    domain_name: &str,
    dns_record_set: Vec<DnsRecord>,
  ) -> Self {
    Self {
      action: Action::UpdateDnsZone,
      param: Param {
        domain_name: domain_name.to_string(),
        customer_number: session.customer_number,
        api_key: session.api_key.to_string(),
        api_session_id: session.api_session_id().to_string(),
        dns_record_set: dns_record_set,
      },
    }
  }
}
