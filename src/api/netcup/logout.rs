use serde::{Deserialize, Serialize};

use super::{Action, SessionCredentials};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
  action: Action,
  #[serde(rename = "param")]
  params: SessionCredentials,
}

impl Request {
  pub fn new(customer_number: u32, api_key: &str, app_session_id: &str) -> Self {
    Self {
      action: Action::Logout,
      params: SessionCredentials {
        customer_number,
        api_key: api_key.to_string(),
        api_session_id: app_session_id.to_string(),
      },
    }
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseData {}

#[cfg(test)]
mod test {
  use super::*;
  use error_stack::{IntoReport, Result, ResultExt};

  use crate::{
    api::netcup::{Action, Response, Status, StatusCode},
    errors::Errors,
  };

  const SUCCESSFUL_LOGOUT_RESPONSE: &str = r#"{
    "serverrequestid": "SUPERSECRETSERVERREQUESTID",
    "clientrequestid": "",
    "action": "logout",
    "status": "success",
    "statuscode": 2000,
    "shortmessage": "Logout successful",
    "longmessage": "Session has been terminated successful.",
    "responsedata": ""
  }"#;

  #[test]
  fn serialize_successful_logout_response() -> Result<(), Errors> {
    let ser = serde_json::from_str::<Response<ResponseData>>(SUCCESSFUL_LOGOUT_RESPONSE)
      .into_report()
      .change_context(Errors::SerializeResponse)?;

    assert_eq!("SUPERSECRETSERVERREQUESTID", ser.server_request_id);
    assert_eq!(None, ser.client_request_id);
    assert_eq!(Some(Action::Logout), ser.action);
    assert_eq!(Status::Success, ser.status);
    assert_eq!(StatusCode::Success, ser.status_code);
    assert_eq!("Logout successful", ser.short_message);
    assert_eq!(
      Some("Session has been terminated successful.".to_string()),
      ser.long_message
    );
    assert_eq!(None, ser.response_data);

    Ok(())
  }
}
