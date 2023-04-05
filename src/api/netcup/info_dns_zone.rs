use error_stack::ResultExt;

use crate::{api, errors::Errors};

use super::{
  models::{
    info_dns_zone::{Params, ResponseData},
    ApiSessionId, Request, Response,
  },
  Action, Client,
};

impl Client<ApiSessionId> {
  pub async fn info_dns_zone(
    &self,
    domain_name: impl Into<String>,
  ) -> error_stack::Result<Response<ResponseData>, Errors> {
    api::netcup::request::<Params, ResponseData>(
      &self.api_url,
      &self.client,
      &Request::new(
        Action::InfoDnsZone,
        Params::new(domain_name.into(), &self.session_credentials),
      ),
    )
    .await
    .change_context(Errors::Login)
  }
}
