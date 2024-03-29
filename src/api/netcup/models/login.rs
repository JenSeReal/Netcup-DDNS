use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseData {
  #[serde(rename = "apisessionid")]
  api_session_id: String,
}

impl ResponseData {
  pub fn api_session_id(&self) -> &str {
    &self.api_session_id
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
  #[serde(rename = "customernumber")]
  customer_number: u32,
  #[serde(rename = "apikey")]
  api_key: String,
  #[serde(rename = "apipassword")]
  api_password: String,
}

impl Params {
  pub fn new(
    customer_number: u32,
    api_key: impl Into<String>,
    api_password: impl Into<String>,
  ) -> Self {
    Self {
      customer_number,
      api_key: api_key.into(),
      api_password: api_password.into(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use error_stack::{IntoReport, Result, ResultExt};

  use crate::{
    api::netcup::{Action, Response, Status, StatusCode},
    errors::Errors,
  };

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
    let ser = serde_json::from_str::<Response<ResponseData>>(SUCCESSFUL_LOGIN)
      .into_report()
      .change_context(Errors::SerializeResponse)?;

    assert_eq!("SUPERSECRETSERVERREQUESTID", ser.server_request_id);
    assert_eq!(None, ser.client_request_id);
    assert_eq!(Some(Action::Login), ser.action);
    assert_eq!(Status::Success, ser.status);
    assert_eq!(StatusCode::Success, ser.status_code);
    assert_eq!("Login successful", ser.short_message);
    assert_eq!(
      Some("Session has been created successful.".to_string()),
      ser.long_message
    );
    assert_eq!(
      Some(ResponseData {
        api_session_id: "SUPERSECRETAPISESSIONID".to_string()
      }),
      ser.response_data
    );

    Ok(())
  }

  #[test]
  fn serialize_failed_login_response() -> Result<(), Errors> {
    let ser = serde_json::from_str::<Response<ResponseData>>(FAILED_LOGIN)
      .into_report()
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
