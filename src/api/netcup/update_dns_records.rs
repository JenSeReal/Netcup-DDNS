use error_stack::ResultExt;

use crate::{api, errors::Errors};

use super::{
  models::{
    update_dns_records::{Params, ResponseData},
    ApiSessionId, DnsRecord, Request, Response,
  },
  Action, Client,
};

impl Client<ApiSessionId> {
  pub async fn update_dns_records(
    &self,
    domain_name: impl Into<String>,
    dns_records: Vec<DnsRecord>,
  ) -> error_stack::Result<Response<ResponseData>, Errors> {
    let domain_name = domain_name.into();
    api::netcup::request(
      &self.api_url,
      &self.client,
      &Request::new(
        Action::UpdateDnsRecords,
        Params::new(domain_name.clone(), &self.session_credentials, dns_records),
      ),
    )
    .await
    .change_context(Errors::UpdateDNSRecords(domain_name))
  }
}
