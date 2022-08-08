use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub version: String,
    pub next_major_version_scheduled: Option<String>,
    pub build_revision: Option<String>,
    pub relays_published: String,
    pub relays_skipped: Option<u64>,
    pub relays_truncated: Option<u64>,
    pub bridges_published: String,
    pub bridges_skipped: Option<u64>,
    pub bridges_truncated: Option<u64>
}