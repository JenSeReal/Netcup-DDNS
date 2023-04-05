use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
  #[error("Could not login into the Netcup API.")]
  Login,
  #[error("Could not logout of the Netcup API.")]
  Logout,
  #[error("Failed to retrieve the API session id.")]
  RetrieveAPISesionId,
  #[error("Failed to send the request to Netcup.")]
  SendRequest,
  #[error("Failed to serialize the response.")]
  SerializeResponse,
  #[error("Failed to load .env file.")]
  LoadingEnvFile,
  #[error("Request failed.")]
  ValidationError,
  #[error("Failed to serialize the domains.")]
  SerializeDomains,
  #[error("Could not find DNS Zone {0}")]
  DNSZoneNotFound(String),
  #[error("Could not update dns zone {0}")]
  UpdateDNSZone(String),
  #[error("Invalid record type {0}")]
  InvalidRecordType(String),
}
