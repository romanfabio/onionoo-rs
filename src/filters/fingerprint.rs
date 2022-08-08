use crate::types::fingerprint::Fingerprint;

use super::traits::IntoQueryFilter;


impl IntoQueryFilter for Fingerprint {
    fn into_query(&self) -> String {
        format!("lookup={}", self.to_string())
    }
}

