use crate::models::{DiscoveredParam, ParamSource};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashSet;
use url::Url;

#[derive(Deserialize)]
struct OtxResponse {
    url_list: Option<Vec<OtxUrlItem>>,
}

#[derive(Deserialize)]
struct OtxUrlItem {
    url: Option<String>,
}

pub struct OtxMiner {
    client: Client,
}

impl OtxMiner {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn fetch_urls(&self, domain: &str) -> Result<HashSet<String>, reqwest::Error> {
        let clean_domain = domain
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');

        let otx_api = format!(
            "https://otx.alienvault.com/api/v1/indicators/domain/{}/url_list?limit=500&page=1",
            clean_domain
        );

        let resp = self.client.get(&otx_api).send().await?;
        if !resp.status().is_success() {
            return Ok(HashSet::new());
        }

        let parsed: OtxResponse = match resp.json().await {
            Ok(data) => data,
            Err(_) => return Ok(HashSet::new()),
        };

        let mut discovered = HashSet::new();
        if let Some(list) = parsed.url_list {
            for item in list {
                if let Some(u) = item.url {
                    discovered.insert(u);
                }
            }
        }

        Ok(discovered)
    }

    pub fn extract_params_from_urls(urls: &HashSet<String>) -> Vec<(String, String, DiscoveredParam)> {
        let mut results = Vec::new();

        for url_str in urls {
            if let Ok(parsed) = Url::parse(url_str) {
                let base = format!(
                    "{}://{}",
                    parsed.scheme(),
                    parsed.host_str().unwrap_or_default()
                );
                let path = parsed.path().to_string();

                for (name, val) in parsed.query_pairs() {
                    let param_name = name.to_string();
                    if !param_name.trim().is_empty() {
                        results.push((
                            base.clone(),
                            path.clone(),
                            DiscoveredParam {
                                name: param_name,
                                source: ParamSource::Otx,
                                sample_value: if val.is_empty() { None } else { Some(val.to_string()) },
                            },
                        ));
                    }
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_otx_params() {
        let mut urls = HashSet::new();
        urls.insert("https://target.com/profile?user_id=1337&tab=security".to_string());

        let extracted = OtxMiner::extract_params_from_urls(&urls);
        assert_eq!(extracted.len(), 2);
        let names: Vec<_> = extracted.iter().map(|(_, _, p)| p.name.as_str()).collect();
        assert!(names.contains(&"user_id"));
        assert!(names.contains(&"tab"));
    }
}
