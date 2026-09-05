use crate::crawler::html_parser::HtmlParser;
use crate::models::DiscoveredParam;
use reqwest::Client;
use std::collections::{HashSet, VecDeque};
use url::Url;

pub struct SpiderResult {
    pub params: Vec<(String, String, DiscoveredParam)>,
    pub js_script_urls: HashSet<String>,
    pub inline_scripts: Vec<(String, String, String)>,
    #[allow(dead_code)]
    pub crawled_urls: HashSet<String>,
}

pub struct SpiderEngine {
    client: Client,
    max_depth: usize,
    max_pages: usize,
}

impl SpiderEngine {
    pub fn new(client: Client, max_depth: usize, max_pages: usize) -> Self {
        Self {
            client,
            max_depth: max_depth.clamp(1, 5),
            max_pages: if max_pages == 0 { 50 } else { max_pages },
        }
    }

    pub async fn crawl(&self, start_url: &str) -> SpiderResult {
        let mut params = Vec::new();
        let mut js_script_urls = HashSet::new();
        let mut inline_scripts = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        let root_host = match Url::parse(start_url).ok().and_then(|u| u.host_str().map(|h| h.to_string())) {
            Some(h) => h,
            None => {
                return SpiderResult {
                    params,
                    js_script_urls,
                    inline_scripts,
                    crawled_urls: visited,
                }
            }
        };

        queue.push_back((start_url.to_string(), 0));

        while let Some((current_url, depth)) = queue.pop_front() {
            if visited.contains(&current_url) || visited.len() >= self.max_pages {
                continue;
            }

            visited.insert(current_url.clone());

            if Self::is_media_or_static(&current_url) {
                continue;
            }

            let resp = match self.client.get(&current_url).send().await {
                Ok(r) => r,
                Err(_) => continue,
            };

            if !resp.status().is_success() {
                continue;
            }

            let content_type = resp
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default();

            if !content_type.contains("text/html") && !content_type.is_empty() {
                continue;
            }

            let body = match resp.text().await {
                Ok(b) => b,
                Err(_) => continue,
            };

            let parsed = HtmlParser::parse_html(&current_url, &body);
            params.extend(parsed.params);
            js_script_urls.extend(parsed.js_script_urls);

            if let Ok(u) = Url::parse(&current_url) {
                let base = format!("{}://{}", u.scheme(), u.host_str().unwrap_or_default());
                let path = u.path().to_string();
                for inline in parsed.inline_scripts {
                    inline_scripts.push((base.clone(), path.clone(), inline));
                }
            }

            // Push next internal links to queue if depth permits
            if depth < self.max_depth && visited.len() < self.max_pages {
                for link in parsed.internal_links {
                    if let Ok(link_parsed) = Url::parse(&link) {
                        if let Some(link_host) = link_parsed.host_str() {
                            if link_host == root_host && !visited.contains(&link) {
                                queue.push_back((link, depth + 1));
                            }
                        }
                    }
                }
            }
        }

        SpiderResult {
            params,
            js_script_urls,
            inline_scripts,
            crawled_urls: visited,
        }
    }

    fn is_media_or_static(url_str: &str) -> bool {
        let ignored_extensions = [
            ".png", ".jpg", ".jpeg", ".gif", ".ico", ".svg", ".webp",
            ".css", ".woff", ".woff2", ".ttf", ".eot",
            ".mp4", ".webm", ".mp3", ".pdf", ".zip", ".tar.gz",
        ];
        let lower = url_str.to_lowercase();
        let path = lower.split('?').next().unwrap_or(&lower);
        ignored_extensions.iter().any(|ext| path.ends_with(ext))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_media_or_static() {
        assert!(SpiderEngine::is_media_or_static("https://example.com/logo.PNG"));
        assert!(SpiderEngine::is_media_or_static("https://example.com/style.css?v=1"));
        assert!(!SpiderEngine::is_media_or_static("https://example.com/products?cat=2"));
    }
}
