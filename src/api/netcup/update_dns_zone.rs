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
      action: Action::InfoDnsZone,
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

#[cfg(test)]
mod test {
  use error_stack::{IntoReport, ResultExt};

  use super::*;
  use crate::{
    api::netcup::{Action, Status, StatusCode},
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
    let ser = serde_json::from_str::<Response>(SUCCESSFUL_REQUEST)
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
    let ser = serde_json::from_str::<Response>(INVALID_API_KEY)
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
    let ser = serde_json::from_str::<Response>(INVALID_DNS_ZONE)
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
