use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

use super::{ApiSessionId, SessionCredentials};

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

impl ResponseData {
  pub fn ttl(&self) -> u32 {
    self.ttl
  }

  pub fn ttl_mut(&mut self, ttl: u32) {
    self.ttl = ttl
  }
}

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
  pub fn new(domain_name: impl Into<String>, session: &SessionCredentials<ApiSessionId>) -> Self {
    Self {
      domain_name: domain_name.into(),
      customer_number: session.customer_number,
      api_key: session.api_key().into(),
      api_session_id: session.api_session_id().into(),
    }
  }
}

#[cfg(test)]
mod test {
  use error_stack::{IntoReport, ResultExt};

  use super::*;
  use crate::{
    api::netcup::{Action, Response, Status, StatusCode},
    errors::Errors,
  };

  const SUCCESSFUL_REQUEST: &str = r#"{
  "serverrequestid": "SUPERSECRETSERVERREQUESTID",
  "clientrequestid": "",
  "action": "infoDnsZone",
  "status": "success",
  "statuscode": 2000,
  "shortmessage": "DNS zone found",
  "longmessage": "DNS zone was found.",
  "responsedata": {
    "name": "example.domain",
    "ttl": "86400",
    "serial": "2022071101",
    "refresh": "28800",
    "retry": "7200",
    "expire": "1209600",
    "dnssecstatus": false
  }
}"#;

  const INVALID_API_KEY: &str = r#"{
  "serverrequestid": "SUPERSECRETSERVERREQUESTID",
  "clientrequestid": "",
  "action": "infoDnsZone",
  "status": "error",
  "statuscode": 4001,
  "shortmessage": "Api session id in invalid format",
  "longmessage": "The session id is not in a valid format.",
  "responsedata": ""
}"#;

  const INVALID_DNS_ZONE: &str = r#"{
  "serverrequestid": "SUPERSECRETSERVERREQUESTID",
  "clientrequestid": "",
  "action": "",
  "status": "error",
  "statuscode": 4013,
  "shortmessage": "Validation Error.",
  "longmessage": "Value in field domainname does not match requirements of type: domainname. ",
  "responsedata": ""
}"#;

  #[test]
  fn serialize_successful_request() -> error_stack::Result<(), Errors> {
    let ser = serde_json::from_str::<Response<ResponseData>>(SUCCESSFUL_REQUEST)
      .into_report()
      .change_context(Errors::SerializeResponse)?;

    assert_eq!("SUPERSECRETSERVERREQUESTID", ser.server_request_id);
    assert_eq!(None, ser.client_request_id);
    assert_eq!(Some(Action::InfoDnsZone), ser.action);
    assert_eq!(Status::Success, ser.status);
    assert_eq!(StatusCode::Success, ser.status_code);
    assert_eq!("DNS zone found", ser.short_message);
    assert_eq!(Some("DNS zone was found.".to_string()), ser.long_message);
    assert_eq!(
      Some(ResponseData {
        name: "example.domain".to_string(),
        ttl: 86400,
        serial: 2022071101,
        refresh: 28800,
        retry: 7200,
        expire: 1209600,
        dns_sec_status: false
      }),
      ser.response_data
    );

    Ok(())
  }

  #[test]
  fn serialize_invalid_api_key() -> error_stack::Result<(), Errors> {
    let ser = serde_json::from_str::<Response<ResponseData>>(INVALID_API_KEY)
      .into_report()
      .change_context(Errors::SerializeResponse)?;

    assert_eq!("SUPERSECRETSERVERREQUESTID", ser.server_request_id);
    assert_eq!(None, ser.client_request_id);
    assert_eq!(Some(Action::InfoDnsZone), ser.action);
    assert_eq!(Status::Error, ser.status);
    assert_eq!(StatusCode::Error, ser.status_code);
    assert_eq!("Api session id in invalid format", ser.short_message);
    assert_eq!(
      Some("The session id is not in a valid format.".to_string()),
      ser.long_message
    );
    assert_eq!(None, ser.response_data);

    Ok(())
  }

  #[test]
  fn serialize_invalid_dns_zone() -> error_stack::Result<(), Errors> {
    let ser = serde_json::from_str::<Response<ResponseData>>(INVALID_DNS_ZONE)
      .into_report()
      .change_context(Errors::SerializeResponse)?;

    assert_eq!("SUPERSECRETSERVERREQUESTID", ser.server_request_id);
    assert_eq!(None, ser.client_request_id);
    assert_eq!(None, ser.action);
    assert_eq!(Status::Error, ser.status);
    assert_eq!(StatusCode::ValidationError, ser.status_code);
    assert_eq!("Validation Error.", ser.short_message);
    assert_eq!(
      Some(
        "Value in field domainname does not match requirements of type: domainname. ".to_string()
      ),
      ser.long_message
    );
    assert_eq!(None, ser.response_data);

    Ok(())
  }
}
