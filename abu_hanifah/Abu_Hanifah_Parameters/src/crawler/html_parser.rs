use crate::models::{DiscoveredParam, ParamSource};
use scraper::{Html, Selector};
use std::collections::HashSet;
use url::Url;

pub struct HtmlParser;

pub struct ExtractedPageContent {
    pub params: Vec<(String, String, DiscoveredParam)>,
    pub js_script_urls: HashSet<String>,
    pub inline_scripts: Vec<String>,
    pub internal_links: HashSet<String>,
}

impl HtmlParser {
    pub fn parse_html(base_url_str: &str, html_content: &str) -> ExtractedPageContent {
        let base_url = Url::parse(base_url_str).ok();
        let document = Html::parse_document(html_content);

        let mut params = Vec::new();
        let mut js_script_urls = HashSet::new();
        let mut inline_scripts = Vec::new();
        let mut internal_links = HashSet::new();

        // 1. Extract inputs grouped by form action if available
        if let Ok(form_selector) = Selector::parse("form") {
            for form in document.select(&form_selector) {
                let form_action = form.value().attr("action").unwrap_or("");
                let resolved_form_url = base_url.as_ref().and_then(|b| b.join(form_action).ok());
                let (form_base, form_path) = Self::resolve_endpoint(&resolved_form_url.or_else(|| base_url.clone()));

                if let Ok(input_sel) = Selector::parse("input, select, textarea") {
                    for element in form.select(&input_sel) {
                        if let Some(name) = element.value().attr("name") {
                            let trimmed = name.trim();
                            if trimmed.is_empty() {
                                continue;
                            }
                            let input_type = element.value().attr("type").unwrap_or("text");
                            let sample_val = element.value().attr("value").map(|v| v.to_string());
                            let source = if input_type.eq_ignore_ascii_case("hidden") {
                                ParamSource::HtmlHidden
                            } else {
                                ParamSource::HtmlInput
                            };
                            params.push((
                                form_base.clone(),
                                form_path.clone(),
                                DiscoveredParam {
                                    name: trimmed.to_string(),
                                    source,
                                    sample_value: sample_val,
                                },
                            ));
                        }
                    }
                }
            }
        }

        // 2. Extract standalone inputs not inside forms
        if let Ok(input_selector) = Selector::parse("input:not(form input), select:not(form select), textarea:not(form textarea)") {
            let (base, path) = Self::resolve_endpoint(&base_url);
            for element in document.select(&input_selector) {
                if let Some(name) = element.value().attr("name") {
                    let trimmed = name.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    let input_type = element.value().attr("type").unwrap_or("text");
                    let sample_val = element.value().attr("value").map(|v| v.to_string());
                    let source = if input_type.eq_ignore_ascii_case("hidden") {
                        ParamSource::HtmlHidden
                    } else {
                        ParamSource::HtmlInput
                    };
                    params.push((
                        base.clone(),
                        path.clone(),
                        DiscoveredParam {
                            name: trimmed.to_string(),
                            source,
                            sample_value: sample_val,
                        },
                    ));
                }
            }
        }

        // 3. Extract links, query parameters, and queue internal links
        if let Ok(a_selector) = Selector::parse("a[href]") {
            for element in document.select(&a_selector) {
                if let Some(href) = element.value().attr("href") {
                    if let Some(ref base) = base_url {
                        if let Ok(resolved) = base.join(href) {
                            let base_str = format!(
                                "{}://{}",
                                resolved.scheme(),
                                resolved.host_str().unwrap_or_default()
                            );
                            let path = resolved.path().to_string();

                            // Track internal link if host matches
                            if resolved.host_str() == base.host_str() {
                                internal_links.insert(resolved.to_string());
                            }

                            for (name, val) in resolved.query_pairs() {
                                if !name.trim().is_empty() {
                                    params.push((
                                        base_str.clone(),
                                        path.clone(),
                                        DiscoveredParam {
                                            name: name.to_string(),
                                            source: ParamSource::QueryString,
                                            sample_value: if val.is_empty() { None } else { Some(val.to_string()) },
                                        },
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        // 4. Extract external and inline JavaScript
        if let Ok(script_selector) = Selector::parse("script") {
            for element in document.select(&script_selector) {
                if let Some(src) = element.value().attr("src") {
                    if let Some(ref base) = base_url {
                        if let Ok(resolved) = base.join(src) {
                            js_script_urls.insert(resolved.to_string());
                        }
                    }
                } else {
                    let script_text: String = element.text().collect();
                    if !script_text.trim().is_empty() {
                        inline_scripts.push(script_text);
                    }
                }
            }
        }

        ExtractedPageContent {
            params,
            js_script_urls,
            inline_scripts,
            internal_links,
        }
    }

    fn resolve_endpoint(base_url: &Option<Url>) -> (String, String) {
        if let Some(ref u) = base_url {
            let base_str = format!("{}://{}", u.scheme(), u.host_str().unwrap_or_default());
            (base_str, u.path().to_string())
        } else {
            ("http://localhost".to_string(), "/".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html_inputs_and_scripts() {
        let html = r#"
            <!DOCTYPE html>
            <html>
            <body>
                <form action="/login" method="POST">
                    <input type="text" name="username" value="admin">
                    <input type="hidden" name="csrf_token" value="xyz999">
                    <input type="hidden" name="debug_mode" value="true">
                </form>
                <a href="/search?query=hello&category=sec">Search</a>
                <script src="/static/js/app.bundle.js"></script>
            </body>
            </html>
        "#;

        let content = HtmlParser::parse_html("https://target.com/page", html);
        let param_names: Vec<_> = content.params.iter().map(|(_, _, p)| p.name.as_str()).collect();

        assert!(param_names.contains(&"username"));
        assert!(param_names.contains(&"csrf_token"));
        assert!(param_names.contains(&"debug_mode"));
        assert!(param_names.contains(&"query"));
        assert!(param_names.contains(&"category"));

        assert_eq!(content.js_script_urls.len(), 1);
        assert!(content
            .js_script_urls
            .contains("https://target.com/static/js/app.bundle.js"));
    }

    #[test]
    fn test_inline_scripts_and_form_action() {
        let html = r#"
            <form action="/auth/verify">
                <input name="otp" value="123456">
            </form>
            <script>
                console.log("inline script running");
            </script>
        "#;
        let content = HtmlParser::parse_html("https://target.com/dashboard", html);
        assert_eq!(content.inline_scripts.len(), 1);
        assert!(content.inline_scripts[0].contains("inline script running"));
        let (base, path, p) = &content.params[0];
        assert_eq!(base, "https://target.com");
        assert_eq!(path, "/auth/verify");
        assert_eq!(p.name, "otp");
    }
}
