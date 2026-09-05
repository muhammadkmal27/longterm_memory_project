use crate::models::{DiscoveredParam, ParamSource};
use reqwest::Client;
use std::collections::HashSet;
use url::Url;

pub struct WaybackMiner {
    client: Client,
}

impl WaybackMiner {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn fetch_urls(&self, domain: &str) -> Result<HashSet<String>, reqwest::Error> {
        let clean_domain = domain
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');

        let cdx_url = format!(
            "https://web.archive.org/cdx/search/cdx?url=*.{}/*&output=json&fl=original&limit=5000",
            clean_domain
        );

        let resp = self.client.get(&cdx_url).send().await?;
        if !resp.status().is_success() {
            return Ok(HashSet::new());
        }

        let raw_records: Vec<Vec<String>> = match resp.json().await {
            Ok(json) => json,
            Err(_) => return Ok(HashSet::new()),
        };

        let mut discovered_urls = HashSet::new();
        for (i, row) in raw_records.into_iter().enumerate() {
            if i == 0 {
                continue; // Skip the CDX header row ["original"]
            }
            if let Some(url_str) = row.into_iter().next() {
                if !Self::is_ignored_extension(&url_str) {
                    discovered_urls.insert(url_str);
                }
            }
        }

        Ok(discovered_urls)
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
                                source: ParamSource::Wayback,
                                sample_value: if val.is_empty() { None } else { Some(val.to_string()) },
                            },
                        ));
                    }
                }
            }
        }

        results
    }

    fn is_ignored_extension(url: &str) -> bool {
        let ignored = [
            ".png", ".jpg", ".jpeg", ".gif", ".css", ".ico", ".svg", ".woff",
            ".woff2", ".ttf", ".eot", ".mp4", ".mp3", ".pdf", ".zip", ".tar.gz",
        ];
        let lower = url.to_lowercase();
        ignored.iter().any(|ext| {
            if let Some(idx) = lower.find('?') {
                lower[..idx].ends_with(ext)
            } else {
                lower.ends_with(ext)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_wayback_params() {
        let mut urls = HashSet::new();
        urls.insert("https://target.com/search?keyword=test&page=1".to_string());
        urls.insert("https://target.com/image.png?v=123".to_string());

        let params = WaybackMiner::extract_params_from_urls(&urls);
        let param_names: Vec<_> = params.iter().map(|(_, _, p)| p.name.as_str()).collect();

        assert!(param_names.contains(&"keyword"));
        assert!(param_names.contains(&"page"));
    }

    #[test]
    fn test_ignored_extensions() {
        assert!(WaybackMiner::is_ignored_extension("https://test.com/logo.png"));
        assert!(WaybackMiner::is_ignored_extension("https://test.com/font.woff2?v=1"));
        assert!(!WaybackMiner::is_ignored_extension("https://test.com/api/users?id=10"));
    }
}
