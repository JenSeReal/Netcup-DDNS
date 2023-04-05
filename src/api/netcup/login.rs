use error_stack::{Report, ResultExt};
use log::debug;

use crate::{api, errors::Errors};

use super::{
  models::{
    login::{Params, ResponseData},
    ApiSessionId, NoApiSessionId, Request,
  },
  Action, Client,
};

impl Client<NoApiSessionId> {
  pub async fn login(self) -> error_stack::Result<Client<ApiSessionId>, Errors> {
    let res = api::netcup::request::<Params, ResponseData>(
      &self.api_url,
      &self.client,
      &Request::new(
        Action::Login,
        Params::new(
          self.session_credentials.customer_number(),
          self.session_credentials.api_key(),
          self.session_credentials.api_password(),
        ),
      ),
    )
    .await
    .change_context(Errors::Login)?;

    let api_session_id = res
      .response_data()
      .map(|data| data.api_session_id())
      .ok_or_else(|| Report::new(Errors::RetrieveAPISesionId))?;

    debug!("API session id: {api_session_id:#?}");

    Ok(Client::<ApiSessionId> {
      client: self.client,
      api_url: self.api_url,
      session_credentials: self.session_credentials.api_session_id(api_session_id),
    })
  }
}
