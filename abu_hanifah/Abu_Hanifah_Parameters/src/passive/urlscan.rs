use crate::models::{DiscoveredParam, ParamSource};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashSet;
use url::Url;

pub struct UrlScanMiner {
    client: Client,
}

#[derive(Deserialize, Debug)]
struct UrlScanResponse {
    results: Option<Vec<UrlScanResultItem>>,
}

#[derive(Deserialize, Debug)]
struct UrlScanResultItem {
    page: Option<UrlScanPage>,
    task: Option<UrlScanTask>,
}

#[derive(Deserialize, Debug)]
struct UrlScanPage {
    url: Option<String>,
}

#[derive(Deserialize, Debug)]
struct UrlScanTask {
    url: Option<String>,
}

impl UrlScanMiner {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn fetch_urls(&self, domain: &str) -> Result<HashSet<String>, reqwest::Error> {
        let clean_domain = domain
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');

        let search_url = format!(
            "https://urlscan.io/api/v1/search/?q=domain:{}&size=100",
            clean_domain
        );

        let resp = self.client.get(&search_url).send().await?;
        if !resp.status().is_success() {
            return Ok(HashSet::new());
        }

        let api_data: UrlScanResponse = match resp.json().await {
            Ok(data) => data,
            Err(_) => return Ok(HashSet::new()),
        };

        let mut discovered = HashSet::new();
        if let Some(results) = api_data.results {
            for item in results {
                if let Some(page) = item.page {
                    if let Some(u) = page.url {
                        discovered.insert(u);
                    }
                }
                if let Some(task) = item.task {
                    if let Some(u) = task.url {
                        discovered.insert(u);
                    }
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
                                source: ParamSource::UrlScan,
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
    fn test_extract_urlscan_params() {
        let mut urls = HashSet::new();
        urls.insert("https://target.com/profile?user_id=1337&token=xyz".to_string());
        urls.insert("https://target.com/assets/app.js".to_string());

        let params = UrlScanMiner::extract_params_from_urls(&urls);
        let names: Vec<_> = params.iter().map(|(_, _, p)| p.name.as_str()).collect();

        assert!(names.contains(&"user_id"));
        assert!(names.contains(&"token"));
        assert_eq!(params[0].2.source, ParamSource::UrlScan);
    }
}
