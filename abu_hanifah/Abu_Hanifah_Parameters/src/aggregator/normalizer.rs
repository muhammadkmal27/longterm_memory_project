use crate::models::{AggregatedEndpoint, DiscoveredParam};
use std::collections::{HashMap, HashSet};
use url::Url;

pub struct ParamAggregator;

impl ParamAggregator {
    /// Normalizes and aggregates parameters discovered across all sources
    pub fn aggregate(
        raw_items: Vec<(String, String, DiscoveredParam)>,
    ) -> Vec<AggregatedEndpoint> {
        let mut endpoint_map: HashMap<(String, String), HashSet<String>> = HashMap::new();

        for (base, path, param) in raw_items {
            let normalized_base = Self::normalize_base(&base);
            let normalized_path = Self::normalize_path(&path);

            let entry = endpoint_map
                .entry((normalized_base, normalized_path))
                .or_insert_with(HashSet::new);

            let clean_name = param.name.trim().to_string();
            if !clean_name.is_empty() {
                entry.insert(clean_name);
            }
        }

        let mut aggregated: Vec<AggregatedEndpoint> = endpoint_map
            .into_iter()
            .map(|((base, path), params)| AggregatedEndpoint {
                base_url: base,
                path,
                params,
            })
            .collect();

        // Sort endpoints by URL for consistent deterministic output
        aggregated.sort_by(|a, b| {
            let full_a = format!("{}{}", a.base_url, a.path);
            let full_b = format!("{}{}", b.base_url, b.path);
            full_a.cmp(&full_b)
        });

        aggregated
    }

    /// Converts a raw URL string and extra parameter list into an AggregatedEndpoint
    #[allow(dead_code)]
    pub fn from_single_url_with_params(url_str: &str, params: Vec<String>) -> Option<AggregatedEndpoint> {
        let parsed = Url::parse(url_str).ok()?;
        let base = format!(
            "{}://{}",
            parsed.scheme(),
            parsed.host_str().unwrap_or_default()
        );
        let path = parsed.path().to_string();

        let mut param_set = HashSet::new();
        for (k, _) in parsed.query_pairs() {
            if !k.trim().is_empty() {
                param_set.insert(k.to_string());
            }
        }

        for p in params {
            let trimmed = p.trim().to_string();
            if !trimmed.is_empty() {
                param_set.insert(trimmed);
            }
        }

        Some(AggregatedEndpoint {
            base_url: base,
            path,
            params: param_set,
        })
    }

    fn normalize_base(base: &str) -> String {
        let lower = base.to_lowercase();
        lower.trim_end_matches('/').to_string()
    }

    fn normalize_path(path: &str) -> String {
        if path.is_empty() {
            return "/".to_string();
        }
        let clean = if !path.starts_with('/') {
            format!("/{}", path)
        } else {
            path.to_string()
        };

        if clean.len() > 1 && clean.ends_with('/') {
            clean.trim_end_matches('/').to_string()
        } else {
            clean
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ParamSource;

    #[test]
    fn test_param_aggregation_and_dalfox_output() {
        let items = vec![
            (
                "https://Target.com/".to_string(),
                "/search/".to_string(),
                DiscoveredParam {
                    name: "keyword".to_string(),
                    source: ParamSource::Wayback,
                    sample_value: None,
                },
            ),
            (
                "https://target.com".to_string(),
                "/search".to_string(),
                DiscoveredParam {
                    name: "sort".to_string(),
                    source: ParamSource::JsRegex,
                    sample_value: None,
                },
            ),
            (
                "https://target.com".to_string(),
                "/search".to_string(),
                DiscoveredParam {
                    name: "page".to_string(),
                    source: ParamSource::HtmlInput,
                    sample_value: None,
                },
            ),
        ];

        let endpoints = ParamAggregator::aggregate(items);
        assert_eq!(endpoints.len(), 1);

        let ep = &endpoints[0];
        assert_eq!(ep.base_url, "https://target.com");
        assert_eq!(ep.path, "/search");
        assert_eq!(ep.params.len(), 3);

        let dalfox_url = ep.to_dalfox_url();
        assert!(dalfox_url.contains("keyword=FUZZ"));
        assert!(dalfox_url.contains("sort=FUZZ"));
        assert!(dalfox_url.contains("page=FUZZ"));
        assert!(dalfox_url.starts_with("https://target.com/search?"));
    }
}
