use serde::{Deserialize, Serialize};

use crate::serialization::{empty_string_as_none, opt_string_or_struct};

use super::{Actions, Response, ResponseData, SessionCredentials, Status, StatusCode};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutRequest {
  action: Actions,
  #[serde(rename = "param")]
  params: SessionCredentials,
}

impl LogoutRequest {
  pub fn new(customer_number: u32, api_key: &str, app_session_id: &str) -> Self {
    Self {
      action: Actions::Logout,
      params: SessionCredentials {
        customer_number,
        api_key: api_key.to_string(),
        api_session_id: app_session_id.to_string(),
      },
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutResponse {
  #[serde(rename = "serverrequestid")]
  server_request_id: String,
  #[serde(rename = "clientrequestid", deserialize_with = "empty_string_as_none")]
  client_request_id: Option<String>,
  #[serde(deserialize_with = "empty_string_as_none")]
  action: Option<Actions>,
  status: Status,
  #[serde(rename = "statuscode")]
  status_code: StatusCode,
  #[serde(rename = "shortmessage")]
  short_message: String,
  #[serde(rename = "longmessage")]
  long_message: Option<String>,
  #[serde(rename = "responsedata", deserialize_with = "opt_string_or_struct")]
  response_data: Option<ResponseData>,
}

impl Response for LogoutResponse {
  fn status_code(&self) -> StatusCode {
    self.status_code
  }
}

#[cfg(test)]
mod test {
  use error_stack::{IntoReport, Result, ResultExt};

  use crate::{
    api::netcup::{Actions, Status, StatusCode},
    errors::Errors,
  };

  use super::LogoutResponse;

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
    let ser = serde_json::from_str::<LogoutResponse>(SUCCESSFUL_LOGOUT_RESPONSE)
      .report()
      .change_context(Errors::SerializeResponse)?;

    assert_eq!("SUPERSECRETSERVERREQUESTID", ser.server_request_id);
    assert_eq!(None, ser.client_request_id);
    assert_eq!(Some(Actions::Logout), ser.action);
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
