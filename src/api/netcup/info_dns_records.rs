use serde::{Deserialize, Serialize};

use super::{Action, ApiSessionId, SessionCredentials};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseData {}

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
