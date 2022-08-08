use filters::{filter::Filter, pagination::Pagination };

use responses::summary::SummaryResponse;
use serde::de::DeserializeOwned;
use ureq::Error;

use crate::filters::traits::IntoQueryFilter;

pub mod filters;
pub mod responses;
pub mod types;

pub const SUMMARY_URL : &'static str = "https://onionoo.torproject.org/summary";
pub const DETAIL_URL : &'static str = "https://onionoo.torproject.org/details";
pub const BANDWIDTH_URL : &'static str = "https://onionoo.torproject.org/bandwidth";
pub const WEIGHT_URL : &'static str = "https://onionoo.torproject.org/weights";
pub const CLIENT_URL : &'static str = "https://onionoo.torproject.org/clients";
pub const UPTIME_URL : &'static str = "https://onionoo.torproject.org/uptime";

pub struct MetricClient;

#[derive(Debug)]
pub enum FetchError {
    NotModified,
    BadRequest,
    NotAvailable,
    InternalServerError,
    ServiceUnavailable,
    ResponseParseError,
    Other
}







impl MetricClient {
    pub fn fetch_summary(&self, filter: &Filter, page: Option<Pagination>) -> Result<SummaryResponse, FetchError> {

        let url = format!("{}?{}&{}", SUMMARY_URL, filter.to_query_string(), page.unwrap_or_default().into_query());

        MetricClient::fetch(&url)
    }

    fn fetch<T: DeserializeOwned>(url: &str) -> Result<T, FetchError> {
        match ureq::get(url).call() {
            Ok(res) => {
                match res.into_json::<T>() {
                    Ok(obj) => Ok(obj),
                    Err(_) => Err(FetchError::ResponseParseError)
                }
            },
            Err(Error::Status(code, _)) => {
                match code {
                    304 => Err(FetchError::NotModified),
                    400 => Err(FetchError::BadRequest),
                    404 => Err(FetchError::NotAvailable),
                    500 => Err(FetchError::InternalServerError),
                    503 => Err(FetchError::ServiceUnavailable),
                    _ => Err(FetchError::Other)
                }
            },
            _ => Err(FetchError::Other)
        }
    }
}