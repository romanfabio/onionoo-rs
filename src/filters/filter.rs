use crate::types::fingerprint::Fingerprint;

use super::running::Running;

use super::doc_type::DocumentType;
use super::traits::IntoQueryFilter;



#[derive(Default)]
pub struct Filter {
    doc_type: Option<DocumentType>,
    running: Option<Running>,
    fingerprint: Option<Fingerprint>
}

impl Filter {
    pub fn to_query_string(&self) -> String {
        let mut variables = vec![];
        if let Some(ref doc_type) = self.doc_type {
            variables.push(doc_type.into_query());
        }
        
        if let Some(ref running) = self.running {
            variables.push(running.into_query());
        }

        if let Some(ref fingerprint) = self.fingerprint {
            variables.push(fingerprint.into_query());
        }

        variables.join("&")
    }
}



pub struct FilterBuilder {
    filter: Filter
}

impl FilterBuilder {
    pub fn new() -> FilterBuilder {
        FilterBuilder { filter: Filter::default() }
    }

    pub fn with_type(mut self, doc_type: DocumentType) -> FilterBuilder {
        self.filter.doc_type = Some(doc_type.into());
        self
    }

    pub fn with_running(mut self, state: bool) -> FilterBuilder {
        self.filter.running = Some(state.into());
        self
    }

    pub fn with_fingerprint(mut self, hash: Fingerprint) -> FilterBuilder {

        self.filter.fingerprint = Some(hash);
        self
    } 

    pub fn build(self) -> Filter {
        self.filter
    }
}