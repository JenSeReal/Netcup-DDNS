use error_stack::ResultExt;

use crate::{api, errors::Errors};

use super::{
  models::{
    info_dns_zone,
    update_dns_zone::{Params, ResponseData},
    ApiSessionId, Request, Response,
  },
  Action, Client,
};

impl Client<ApiSessionId> {
  pub async fn update_dns_zone(
    &self,
    domain_name: impl Into<String>,
    dns_zone: info_dns_zone::ResponseData,
  ) -> error_stack::Result<Response<ResponseData>, Errors> {
    api::netcup::request::<Params, ResponseData>(
      &self.api_url,
      &self.client,
      &Request::new(
        Action::UpdateDnsRecords,
        Params::new(domain_name, &self.session_credentials, dns_zone),
      ),
    )
    .await
    .change_context(Errors::Login)
  }
}
