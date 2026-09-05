use crate::models::{DiscoveredParam, ParamSource};
use reqwest::Client;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

pub struct ReflectionTester {
    client: Client,
}

impl ReflectionTester {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn generate_canary(param: &str) -> String {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros();
        let short_hash = (ts ^ (param.len() as u128)) & 0xFFFFFF;
        format!("ahx_{}_{:x}", param, short_hash)
    }

    /// Fast batch reflection tester: Injects unique canaries for all params in batch into 1 request
    pub async fn check_batch_reflection(
        &self,
        target_url: &str,
        params: &[String],
        method: &str,
    ) -> Vec<DiscoveredParam> {
        let mut discovered = Vec::new();
        if params.is_empty() {
            return discovered;
        }

        let mut canary_map = HashMap::new();
        for p in params {
            let canary = Self::generate_canary(p);
            canary_map.insert(canary, p.clone());
        }

        let is_post = method.eq_ignore_ascii_case("post");
        let (body_text, header_text) = if is_post {
            self.send_post_probe(target_url, &canary_map).await
        } else {
            self.send_get_probe(target_url, &canary_map).await
        };

        for (canary, param_name) in canary_map {
            if body_text.contains(&canary) || header_text.contains(&canary) {
                discovered.push(DiscoveredParam {
                    name: param_name,
                    source: ParamSource::Reflection,
                    sample_value: Some(canary),
                });
            }
        }

        discovered
    }

    async fn send_get_probe(&self, target_url: &str, canary_map: &HashMap<String, String>) -> (String, String) {
        let mut parsed = match Url::parse(target_url) {
            Ok(u) => u,
            Err(_) => return (String::new(), String::new()),
        };

        {
            let mut pairs = parsed.query_pairs_mut();
            for (canary, param_name) in canary_map {
                pairs.append_pair(param_name, canary);
            }
        }

        match self.client.get(parsed).send().await {
            Ok(resp) => {
                let headers_str = resp
                    .headers()
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or_default()))
                    .collect::<Vec<_>>()
                    .join("\n");
                let body = resp.text().await.unwrap_or_default();
                (body, headers_str)
            }
            Err(_) => (String::new(), String::new()),
        }
    }

    async fn send_post_probe(&self, target_url: &str, canary_map: &HashMap<String, String>) -> (String, String) {
        let mut form_data = Vec::new();
        for (canary, param_name) in canary_map {
            form_data.push((param_name.as_str(), canary.as_str()));
        }

        match self.client.post(target_url).form(&form_data).send().await {
            Ok(resp) => {
                let headers_str = resp
                    .headers()
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or_default()))
                    .collect::<Vec<_>>()
                    .join("\n");
                let body = resp.text().await.unwrap_or_default();
                (body, headers_str)
            }
            Err(_) => (String::new(), String::new()),
        }
    }

    #[allow(dead_code)]
    pub fn is_reflected_in_text(canary: &str, text: &str) -> bool {
        text.contains(canary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canary_generation_and_detection() {
        let canary = ReflectionTester::generate_canary("search");
        assert!(canary.starts_with("ahx_search_"));

        let response_html = format!(r#"<div>Results for: {}</div>"#, canary);
        assert!(ReflectionTester::is_reflected_in_text(&canary, &response_html));

        let other_html = "<div>No results found</div>";
        assert!(!ReflectionTester::is_reflected_in_text(&canary, other_html));
    }
}
