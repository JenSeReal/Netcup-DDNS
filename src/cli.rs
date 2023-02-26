use std::str::FromStr;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::errors::Errors;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DNSEntry {
  domain: String,
  sub_domains: Vec<String>,
}

impl FromStr for DNSEntry {
  type Err = Errors;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.is_empty() {
      return Err(Errors::SerializeDomains);
    }

    let (domain, sub_domains) = match s.split_once(':') {
      Some((domain, sub_domains)) => (
        domain.trim().to_owned(),
        sub_domains
          .split(',')
          .map(|s| s.trim().to_owned())
          .collect(),
      ),
      None => (s.trim().to_owned(), vec![]),
    };

    Ok(Self {
      domain,
      sub_domains,
    })
  }
}

impl DNSEntry {
  pub fn domain(&self) -> &str {
    &self.domain
  }
}

#[derive(Debug, StructOpt)]
#[structopt(
  name = "Netcup updater",
  about = "Updates one DNS record in Netcup via the API."
)]
pub(crate) struct Cli {
  #[structopt(
    short,
    long,
    env = "CUSTOMER_NUMBER",
    help = "The customer number which identifies your Netcup account."
  )]
  customer_number: u32,
  #[structopt(
    short = "u",
    long,
    env = "API_URL",
    default_value = "https://ccp.netcup.net/run/webservice/servers/endpoint.php?JSON",
    help = "The URL of the netcup api."
  )]
  api_url: String,
  #[structopt(
    short = "k",
    long,
    env = "API_KEY",
    help = "The API key generated by Netcup in the CCP."
  )]
  api_key: String,
  #[structopt(
    short = "p",
    long,
    env = "API_PASSWORD",
    help = "The API password generated by Netcup in the CCP."
  )]
  api_password: String,
  #[structopt(
    short,
    long,
    env = "TTL",
    help = "Should the TTL be reduced to a certain time in ms, which could be better for ddns."
  )]
  ttl: Option<u32>,
  #[structopt(env = "DOMAINS", value_delimiter = ";")]
  domains: Vec<DNSEntry>,
}

impl Cli {
  pub(crate) fn customer_number(&self) -> u32 {
    self.customer_number
  }

  pub(crate) fn api_url(&self) -> &str {
    &self.api_url
  }

  pub(crate) fn api_key(&self) -> &str {
    &self.api_key
  }

  pub(crate) fn api_password(&self) -> &str {
    &self.api_password
  }

  pub(crate) fn domains(&self) -> &Vec<DNSEntry> {
    &self.domains
  }

  pub(crate) fn ttl(&self) -> Option<u32> {
    self.ttl
  }
}

#[cfg(test)]
mod test {
  use super::*;

  const VALID_DOMAINS: &str = r#"myfirstdomain.com: server, dddns; myseconddomain.com: @, *, some-subdomain; mythriddomain.com;"#;

  #[test]
  fn serialize_domains() {
    let domains = VALID_DOMAINS
      .split(";")
      .filter(|s| !s.is_empty())
      .filter_map(|s| DNSEntry::from_str(s).ok())
      .collect::<Vec<_>>();

    assert_eq!(domains.len(), 3);
  }
}
