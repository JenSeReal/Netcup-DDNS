use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

use super::Action;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseData {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
  action: Action,
  param: Param,
}

impl Request {
  pub fn new(domain_name: &str, customer_number: u32, api_key: &str, api_session_id: &str) -> Self {
    Self {
      action: Action::InfoDnsRecords,
      param: Param {
        domain_name: domain_name.to_string(),
        customer_number,
        api_key: api_key.to_string(),
        api_session_id: api_session_id.to_string(),
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
