use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Params {
  #[serde(rename = "customernumber")]
  customer_number: u32,
  #[serde(rename = "apikey")]
  api_key: String,
  #[serde(rename = "apisessionid")]
  api_session_id: String,
}

impl Params {
  pub fn new(
    customer_number: u32,
    api_key: impl Into<String>,
    api_session_id: impl Into<String>,
  ) -> Self {
    Self {
      customer_number,
      api_key: api_key.into(),
      api_session_id: api_session_id.into(),
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
