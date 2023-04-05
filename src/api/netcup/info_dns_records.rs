use error_stack::ResultExt;

use crate::{api, errors::Errors};

use super::{
  models::{
    info_dns_records::{Params, ResponseData},
    ApiSessionId, Request, Response,
  },
  Action, Client,
};

impl Client<ApiSessionId> {
  pub async fn info_dns_records(
    &self,
    domain_name: impl Into<String>,
  ) -> error_stack::Result<Response<ResponseData>, Errors> {
    let domain_name: String = domain_name.into();
    api::netcup::request::<Params, ResponseData>(
      &self.api_url,
      &self.client,
      &Request::new(
        Action::InfoDnsRecords,
        Params::new(domain_name.clone(), &self.session_credentials),
      ),
    )
    .await
    .change_context(Errors::DNSZoneNotFound(domain_name))
  }
}
