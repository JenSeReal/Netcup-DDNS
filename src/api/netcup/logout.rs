use error_stack::ResultExt;

use crate::{api, errors::Errors};

use super::{
  models::{
    logout::{Params, ResponseData},
    ApiSessionId, Request,
  },
  Action, Client,
};

impl Client<ApiSessionId> {
  pub async fn logout(self) -> error_stack::Result<(), Errors> {
    api::netcup::request::<Params, ResponseData>(
      &self.api_url,
      &self.client,
      &Request::new(
        Action::Logout,
        Params::new(
          self.session_credentials.customer_number(),
          self.session_credentials.api_key(),
          self.session_credentials.api_password(),
        ),
      ),
    )
    .await
    .change_context(Errors::Logout)?;

    Ok(())
  }
}
