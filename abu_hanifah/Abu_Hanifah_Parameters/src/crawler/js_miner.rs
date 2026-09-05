use crate::models::{DiscoveredParam, ParamSource};
use regex::Regex;
use std::collections::HashSet;

pub struct JsMiner {
    param_regexes: Vec<Regex>,
    js_blacklist: HashSet<&'static str>,
}

impl JsMiner {
    pub fn new() -> Self {
        let regex_patterns = vec![
            // Matches: urlParams.get("query"), searchParams.get('sort'), params.get('q')
            r#"\.get\(['"]([a-zA-Z0-9_\-\.]{1,50})['"]\)"#,
            // Matches: urlParams.has("tab"), searchParams.has('view')
            r#"\.has\(['"]([a-zA-Z0-9_\-\.]{1,50})['"]\)"#,
            // Matches: params.set("key", val) or params.append("key", val)
            r#"\.(?:set|append)\(['"]([a-zA-Z0-9_\-\.]{1,50})['"]"#,
            // Matches: router.query.searchTerm or req.query.searchTerm
            r#"(?:router|req)\.query\.([a-zA-Z0-9_]{1,50})"#,
            // Matches: ?token=, &redirect= inside JS strings or template literals
            r#"[\?&]([a-zA-Z0-9_\-\.]{1,50})=(?:[^\s"'`&]*)"#,
            // Matches: axios.get('/api', { params: { filter: 'x', page: 1 } })
            r#"params\s*:\s*\{([^\}]+)\}"#,
            // Matches: $.ajax({ data: { user: 'a', pass: 'b' } })
            r#"data\s*:\s*\{([^\}]+)\}"#,
        ];

        let param_regexes = regex_patterns
            .into_iter()
            .filter_map(|p| Regex::new(p).ok())
            .collect();

        let mut js_blacklist = HashSet::new();
        let keywords = [
            "function", "return", "const", "let", "var", "if", "else", "true",
            "false", "null", "undefined", "this", "new", "typeof", "instanceof",
            "prototype", "window", "document", "length", "split", "join", "map",
            "filter", "forEach", "indexOf", "includes", "push", "slice", "replace",
            "catch", "finally", "import", "export", "default", "class", "async", "await",
            "then", "status", "headers", "method", "body", "success", "error",
        ];
        for kw in keywords {
            js_blacklist.insert(kw);
        }

        Self {
            param_regexes,
            js_blacklist,
        }
    }

    pub fn extract_params_from_js(
        &self,
        base_url: &str,
        path: &str,
        js_code: &str,
    ) -> Vec<(String, String, DiscoveredParam)> {
        let mut discovered_names = HashSet::new();

        // 1. Run direct extraction regexes
        for re in &self.param_regexes {
            for cap in re.captures_iter(js_code) {
                if let Some(matched) = cap.get(1) {
                    let text = matched.as_str().trim();
                    // If this was an object capture like { foo: 1, bar: 2 }
                    if text.contains(':') || text.contains(',') {
                        for sub_part in text.split(',') {
                            if let Some(key) = sub_part.split(':').next() {
                                let clean_key = key.trim().trim_matches(|c| c == '\'' || c == '"' || c == ' ');
                                if self.is_valid_param_name(clean_key) {
                                    discovered_names.insert(clean_key.to_string());
                                }
                            }
                        }
                    } else if self.is_valid_param_name(text) {
                        discovered_names.insert(text.to_string());
                    }
                }
            }
        }

        discovered_names
            .into_iter()
            .map(|name| {
                (
                    base_url.to_string(),
                    path.to_string(),
                    DiscoveredParam {
                        name,
                        source: ParamSource::JsRegex,
                        sample_value: None,
                    },
                )
            })
            .collect()
    }

    pub fn extract_params_from_inline_js(
        &self,
        base_url: &str,
        path: &str,
        js_code: &str,
    ) -> Vec<(String, String, DiscoveredParam)> {
        let mut results = self.extract_params_from_js(base_url, path, js_code);
        for (_, _, p) in &mut results {
            p.source = ParamSource::InlineJs;
        }
        results
    }

    fn is_valid_param_name(&self, name: &str) -> bool {
        let trimmed = name.trim();
        if trimmed.is_empty() || trimmed.len() > 50 || trimmed.len() < 2 {
            return false;
        }

        if self.js_blacklist.contains(trimmed) {
            return false;
        }

        // Must start with alpha or underscore
        let first_char = trimmed.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return false;
        }

        // Must only contain alphanumerics, underscores, dashes, dots
        trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_js_params() {
        let miner = JsMiner::new();
        let js_code = r#"
            const urlParams = new URLSearchParams(window.location.search);
            const token = urlParams.get('auth_token');
            const hasDebug = urlParams.has("debug_view");
            const filter = router.query.filterCategory;
            fetch('/api/v2/items?page=1&sort=desc');
            axios.get('/endpoint', { params: { secret_key: 'abc', limit: 10 } });
        "#;

        let results = miner.extract_params_from_js("https://target.com", "/app", js_code);
        let names: Vec<_> = results.iter().map(|(_, _, p)| p.name.as_str()).collect();

        assert!(names.contains(&"auth_token"));
        assert!(names.contains(&"debug_view"));
        assert!(names.contains(&"filterCategory"));
        assert!(names.contains(&"sort"));
        assert!(names.contains(&"secret_key"));
        assert!(names.contains(&"limit"));
    }

    #[test]
    fn test_extract_inline_js_params() {
        let miner = JsMiner::new();
        let js_code = r#"params.set('session_id', 'xyz'); req.query.admin_user;"#;
        let results = miner.extract_params_from_inline_js("https://target.com", "/", js_code);
        assert_eq!(results[0].2.source, ParamSource::InlineJs);
        let names: Vec<_> = results.iter().map(|(_, _, p)| p.name.as_str()).collect();
        assert!(names.contains(&"session_id") || names.contains(&"admin_user"));
    }
}
