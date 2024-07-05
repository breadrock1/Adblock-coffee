use adblock::lists::ParseOptions;
use adblock::request::Request;
use adblock::{Engine, FilterSet};

use crate::errors::RustException;

pub(crate) struct AdvtBlocker {
    engine: Engine,
}

impl AdvtBlocker {
    pub fn new(filter_list: Vec<String>) -> Self {
        let debug_info = true;
        let mut filter_set = FilterSet::new(debug_info);
        filter_set.add_filters(&filter_list, ParseOptions::default());

        let filter_engine = Engine::from_filter_set(filter_set, true);

        AdvtBlocker {
            engine: filter_engine,
        }
    }

    pub fn check_network_urls(
        &self,
        url: &str,
        src_url: &str,
        req_type: &str,
    ) -> Result<bool, RustException> {
        let request = Request::new(url, src_url, req_type)
            .map_err(|err| RustException::CreateRequest(err.to_string()))?;

        let blocker_result = self.engine.check_network_request(&request);
        Ok(blocker_result.matched)
    }
}

impl Default for AdvtBlocker {
    fn default() -> Self {
        AdvtBlocker::new(Vec::default())
    }
}

#[cfg(test)]
mod adblock_test {
    use super::*;

    #[test]
    fn check_url() {
        let rules = vec![
            "-advertisement-icon.".to_string(),
            "-advertisement-management/".to_string(),
            "-advertisement.".to_string(),
            "-advertisement/script.".to_string(),
        ];

        let advt_blocker = AdvtBlocker::new(rules);
        let check_result = advt_blocker
            .check_network_urls(
                "http://example.com/-advertisement-icon.",
                "http://example.com/helloworld",
                "image",
            )
            .unwrap();

        assert_eq!(check_result, true);
    }
}
