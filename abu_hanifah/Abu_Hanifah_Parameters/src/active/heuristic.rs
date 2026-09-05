use crate::models::{DiscoveredParam, ParamSource};
use reqwest::Client;
use std::collections::HashSet;
use url::Url;

pub struct HeuristicMiner {
    client: Client,
    chunk_size: usize,
    method: String,
}

#[derive(Debug, Clone)]
struct ResponseFingerprint {
    status: u16,
    content_length: usize,
    headers_count: usize,
}

impl HeuristicMiner {
    pub fn new(client: Client, chunk_size: usize, method: String) -> Self {
        Self {
            client,
            chunk_size: if chunk_size == 0 { 25 } else { chunk_size },
            method,
        }
    }

    pub async fn discover_hidden_params(
        &self,
        target_url: &str,
        wordlist: &[String],
    ) -> Vec<DiscoveredParam> {
        let mut discovered = Vec::new();
        let baseline = match self.get_fingerprint(target_url, &[]).await {
            Some(fp) => fp,
            None => return discovered,
        };

        for chunk in wordlist.chunks(self.chunk_size) {
            if self.probe_chunk(target_url, chunk, &baseline).await {
                // Anomaly detected! Narrow down using dichotomy
                let found = self.dichotomy_search(target_url, chunk, &baseline).await;
                for name in found {
                    discovered.push(DiscoveredParam {
                        name,
                        source: ParamSource::Heuristic,
                        sample_value: Some("canary_val".to_string()),
                    });
                }
            }
        }

        discovered
    }

    async fn probe_chunk(&self, target_url: &str, chunk: &[String], baseline: &ResponseFingerprint) -> bool {
        let params: Vec<(&str, &str)> = chunk.iter().map(|p| (p.as_str(), "1")).collect();
        if let Some(fp) = self.get_fingerprint(target_url, &params).await {
            Self::is_anomaly(&fp, baseline)
        } else {
            false
        }
    }

    async fn dichotomy_search(
        &self,
        target_url: &str,
        slice: &[String],
        baseline: &ResponseFingerprint,
    ) -> HashSet<String> {
        let mut results = HashSet::new();
        if slice.is_empty() {
            return results;
        }

        if slice.len() == 1 {
            let single = &slice[0];
            let params = [(single.as_str(), "1")];
            if let Some(fp) = self.get_fingerprint(target_url, &params).await {
                if Self::is_anomaly(&fp, baseline) {
                    results.insert(single.clone());
                }
            }
            return results;
        }

        let mid = slice.len() / 2;
        let (left, right) = slice.split_at(mid);

        if self.probe_chunk(target_url, left, baseline).await {
            let left_found = Box::pin(self.dichotomy_search(target_url, left, baseline)).await;
            results.extend(left_found);
        }

        if self.probe_chunk(target_url, right, baseline).await {
            let right_found = Box::pin(self.dichotomy_search(target_url, right, baseline)).await;
            results.extend(right_found);
        }

        results
    }

    async fn get_fingerprint(&self, base_url: &str, query_params: &[(&str, &str)]) -> Option<ResponseFingerprint> {
        let resp = if self.method.eq_ignore_ascii_case("post") {
            self.client.post(base_url).form(&query_params).send().await.ok()?
        } else {
            let mut parsed = Url::parse(base_url).ok()?;
            {
                let mut pairs = parsed.query_pairs_mut();
                for (k, v) in query_params {
                    pairs.append_pair(k, v);
                }
            }
            self.client.get(parsed).send().await.ok()?
        };

        let status = resp.status().as_u16();
        let headers_count = resp.headers().len();
        let body = resp.text().await.unwrap_or_default();

        Some(ResponseFingerprint {
            status,
            content_length: body.len(),
            headers_count,
        })
    }

    fn is_anomaly(probe: &ResponseFingerprint, baseline: &ResponseFingerprint) -> bool {
        // Status code deviation (e.g. 200 -> 302, 500, 403, 400)
        if probe.status != baseline.status {
            return true;
        }

        // Significant header change
        if probe.headers_count.abs_diff(baseline.headers_count) >= 2 {
            return true;
        }

        // Body size difference (> 20 bytes delta)
        let delta = probe.content_length.abs_diff(baseline.content_length);
        delta > 20
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anomaly_detection() {
        let baseline = ResponseFingerprint {
            status: 200,
            content_length: 1000,
            headers_count: 10,
        };

        let same = ResponseFingerprint {
            status: 200,
            content_length: 1005,
            headers_count: 10,
        };
        assert!(!HeuristicMiner::is_anomaly(&same, &baseline));

        let status_diff = ResponseFingerprint {
            status: 500,
            content_length: 1000,
            headers_count: 10,
        };
        assert!(HeuristicMiner::is_anomaly(&status_diff, &baseline));

        let length_diff = ResponseFingerprint {
            status: 200,
            content_length: 1300,
            headers_count: 10,
        };
        assert!(HeuristicMiner::is_anomaly(&length_diff, &baseline));
    }
}
