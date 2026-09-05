use crate::models::{DiscoveredParam, ParamSource};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashSet;
use url::Url;

pub struct CommonCrawlMiner {
    client: Client,
}

#[derive(Deserialize, Debug)]
struct IndexEntry {
    id: String,
}

impl CommonCrawlMiner {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn fetch_urls(&self, domain: &str) -> Result<HashSet<String>, reqwest::Error> {
        let clean_domain = domain
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');

        // 1. Get latest index name or fallback to recent CC index
        let index_id = self.get_latest_index().await.unwrap_or_else(|| "CC-MAIN-2024-51-index".to_string());

        // 2. Query Common Crawl index
        let query_url = format!(
            "https://index.commoncrawl.org/{}?url=*.{}/*&output=json&fl=url&limit=3000",
            index_id, clean_domain
        );

        let resp = self.client.get(&query_url).send().await?;
        if !resp.status().is_success() {
            return Ok(HashSet::new());
        }

        let body_text = match resp.text().await {
            Ok(t) => t,
            Err(_) => return Ok(HashSet::new()),
        };

        let mut discovered = HashSet::new();
        // Common Crawl returns NDJSON (newline delimited JSON objects)
        for line in body_text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                if let Some(u) = val.get("url").and_then(|v| v.as_str()) {
                    discovered.insert(u.to_string());
                }
            }
        }

        Ok(discovered)
    }

    async fn get_latest_index(&self) -> Option<String> {
        let resp = self
            .client
            .get("https://index.commoncrawl.org/collinfo.json")
            .send()
            .await
            .ok()?;

        if !resp.status().is_success() {
            return None;
        }

        let indices: Vec<IndexEntry> = resp.json().await.ok()?;
        indices.into_iter().next().map(|i| i.id)
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
                                source: ParamSource::CommonCrawl,
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
    fn test_extract_commoncrawl_params() {
        let mut urls = HashSet::new();
        urls.insert("https://target.com/items?category=books&order=asc".to_string());

        let params = CommonCrawlMiner::extract_params_from_urls(&urls);
        let names: Vec<_> = params.iter().map(|(_, _, p)| p.name.as_str()).collect();

        assert!(names.contains(&"category"));
        assert!(names.contains(&"order"));
        assert_eq!(params[0].2.source, ParamSource::CommonCrawl);
    }
}
