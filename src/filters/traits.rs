pub trait IntoQueryFilter {
    fn into_query(&self) -> String;
}