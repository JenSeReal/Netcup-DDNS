use serde::{Deserialize, Serialize};

use super::{Action, ApiSessionId, SessionCredentials};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
  action: Action,
  #[serde(rename = "param")]
  params: Params,
}

impl Request {
  pub fn new(session: &SessionCredentials<ApiSessionId>) -> Self {
    Self {
      action: Action::Logout,
      params: Params {
        customer_number: session.customer_number,
        api_key: session.api_key.to_string(),
        api_session_id: session.api_session_id().to_string(),
      },
    }
  }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Params {
  customer_number: u32,
  api_key: String,
  api_session_id: String,
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
