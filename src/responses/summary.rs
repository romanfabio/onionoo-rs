use serde::{Deserialize, Deserializer};

use crate::types::fingerprint::Fingerprint;

use super::metadata::Metadata;

#[derive(Deserialize, Debug)]
pub struct SummaryRelay {
    #[serde(rename="n")]
    pub nickname: String,
    #[serde(rename="f")]
    pub fingerprint: Fingerprint,
    #[serde(rename="r")]
    pub running: bool,
    #[serde(rename="a", deserialize_with="from_ips")]
    pub addresses: Vec<std::net::IpAddr>
}

fn from_ips<'de, D>(deserializer: D) -> Result<Vec<std::net::IpAddr>, D::Error>
where
    D: Deserializer<'de>,
{
    let addresses : Vec<String> = Vec::<String>::deserialize(deserializer)?;
    let mut result : Vec<std::net::IpAddr> = Vec::with_capacity(addresses.len());

    for addr in addresses {
        if addr.starts_with("[") {
            let addr = addr.trim_start_matches("[").trim_end_matches("]");
            result.push(addr.parse().map_err(serde::de::Error::custom)?);
        } else {
            result.push(addr.parse().map_err(serde::de::Error::custom)?);
        }
    }
    Ok(result)
}

#[derive(Deserialize, Debug)]
pub struct SummaryBridge {
    #[serde(rename="n")]
    pub nickname: String,
    #[serde(rename="h")]
    pub hash: Fingerprint,
    #[serde(rename="r")]
    pub running: bool
}

#[derive(Deserialize, Debug)]
pub struct SummaryResponse {
    #[serde(flatten)]
    pub metadata: Metadata,
    pub relays: Vec<SummaryRelay>,
    pub bridges: Vec<SummaryBridge>
}