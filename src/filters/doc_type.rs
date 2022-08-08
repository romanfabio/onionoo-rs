use super::traits::IntoQueryFilter;

pub enum DocumentType {
    Relay,
    Bridge
}

impl IntoQueryFilter for DocumentType {
    fn into_query(&self) -> String {
        format!("type={}", match *self {
            DocumentType::Bridge => "bridge",
            DocumentType::Relay => "relay"
        })
    }
}