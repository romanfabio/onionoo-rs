use super::traits::IntoQueryFilter;

pub struct Pagination {
    pub offset: u64,
    pub limit: u64
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination { offset: 0, limit: 10 }
    }
}

impl IntoQueryFilter for Pagination {
    fn into_query(&self) -> String {
        format!("offset={}&limit={}", self.offset, self.limit)
    }
}

