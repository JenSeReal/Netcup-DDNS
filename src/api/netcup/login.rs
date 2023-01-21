use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::serialization::{empty_string_as_none, opt_string_or_struct};

use super::{Actions, LoginCredentials, Response, ResponseData, Status, StatusCode};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
  action: Actions,
  #[serde(rename = "param")]
  params: LoginCredentials,
}

impl LoginRequest {
  pub fn new(customer_number: u32, api_key: &str, api_password: &str) -> Self {
    Self {
      action: Actions::Login,
      params: LoginCredentials {
        customer_number,
        api_key: api_key.to_string(),
        api_password: api_password.to_string(),
      },
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde_as]
pub struct LoginResponse {
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

impl LoginResponse {
  pub fn api_session_id(&self) -> Option<String> {
    Some(self.response_data.as_ref()?.app_session_id.clone())
  }
}

impl Response for LoginResponse {
  fn status_code(&self) -> StatusCode {
    self.status_code
  }
}

#[cfg(test)]
mod tests {
  use error_stack::{IntoReport, Result, ResultExt};

  use crate::{
    api::netcup::{Actions, ResponseData, Status, StatusCode},
    errors::Errors,
  };

  use super::LoginResponse;

  const FAILED_LOGIN: &str = r#"{
    "serverrequestid": "SUPERSECRETSERVERREQUESTID",
    "clientrequestid": "",
    "action": "",
    "status": "error",
    "statuscode": 4013,
    "shortmessage": "Validation Error.",
    "longmessage": "More than 180 requests per minute. Please wait and retry later. Please contact our customer service to find out if the limitation of requests can be increased.",
    "responsedata": ""
  }"#;
  const SUCCESSFUL_LOGIN: &str = r#"{
      "serverrequestid": "SUPERSECRETSERVERREQUESTID",
      "clientrequestid": "",
      "action": "login",
      "status": "success",
      "statuscode": 2000,
      "shortmessage": "Login successful",
      "longmessage": "Session has been created successful.",
      "responsedata": {
        "apisessionid": "SUPERSECRETAPISESSIONID"
      }
    }"#;

  #[test]
  fn serialize_successful_login_response() -> Result<(), Errors> {
    let ser = serde_json::from_str::<LoginResponse>(SUCCESSFUL_LOGIN)
      .report()
      .change_context(Errors::SerializeResponse)?;

    assert_eq!("SUPERSECRETSERVERREQUESTID", ser.server_request_id);
    assert_eq!(None, ser.client_request_id);
    assert_eq!(Some(Actions::Login), ser.action);
    assert_eq!(Status::Success, ser.status);
    assert_eq!(StatusCode::Success, ser.status_code);
    assert_eq!("Login successful", ser.short_message);
    assert_eq!(
      Some("Session has been created successful.".to_string()),
      ser.long_message
    );
    assert_eq!(
      Some(ResponseData {
        app_session_id: "SUPERSECRETAPISESSIONID".to_string()
      }),
      ser.response_data
    );

    Ok(())
  }

  #[test]
  fn serialize_failed_login_response() -> Result<(), Errors> {
    let ser = serde_json::from_str::<LoginResponse>(FAILED_LOGIN)
      .report()
      .change_context(Errors::SerializeResponse)?;

    assert_eq!("SUPERSECRETSERVERREQUESTID", ser.server_request_id);
    assert_eq!(None, ser.client_request_id);
    assert_eq!(None, ser.action);
    assert_eq!(Status::Error, ser.status);
    assert_eq!(StatusCode::ValidationError, ser.status_code);
    assert_eq!("Validation Error.", ser.short_message);
    assert_eq!(
      Some("More than 180 requests per minute. Please wait and retry later. Please contact our customer service to find out if the limitation of requests can be increased.".to_string()),
      ser.long_message
    );
    assert_eq!(None, ser.response_data);

    Ok(())
  }
}
