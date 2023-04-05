use std::net::IpAddr;
pub async fn external() -> Vec<IpAddr> {
  let (ip4, ip6) = tokio::join!(public_ip::addr_v4(), public_ip::addr_v6());
  let ips = vec![ip4.map(IpAddr::V4), ip6.map(IpAddr::V6)];

  ips.into_iter().flatten().collect()
}
