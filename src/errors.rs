use std::fmt::Display;

use error_stack::Context;

#[derive(Debug)]
pub enum Errors {
  Login,
  Logout,
  RetrieveAPISesionId,
  SendRequest,
  SerializeResponse,
  LoadingEnvFile,
  ValidationError,
}

impl Display for Errors {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Errors::Login => write!(f, "Could not login into the Netcup API."),
      Errors::Logout => write!(f, "Could not logout of the Netcup API."),
      Errors::RetrieveAPISesionId => {
        write!(f, "Failed to retrieve the API session id.")
      }
      Errors::SendRequest => write!(f, "Failed to send the request to Netcup."),
      Errors::SerializeResponse => write!(f, "Failed to serialize the response."),
      Errors::LoadingEnvFile => write!(f, "Failed to load .env file."),
      Errors::ValidationError => write!(f, "Request failed."),
    }
  }
}

impl Context for Errors {}
