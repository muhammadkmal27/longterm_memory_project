use crate::models::{DiscoveredParam, ParamSource};
use regex::Regex;
use reqwest::Client;
use std::collections::HashSet;
use url::Url;

pub struct RobotsSitemapHarvester {
    client: Client,
}

impl RobotsSitemapHarvester {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn harvest(&self, target_url: &str) -> (Vec<(String, String, DiscoveredParam)>, HashSet<String>) {
        let mut findings = Vec::new();
        let mut discovered_urls = HashSet::new();

        let root_url = match Self::get_root_url(target_url) {
            Some(u) => u,
            None => return (findings, discovered_urls),
        };

        // 1. Fetch robots.txt
        let robots_url = format!("{}/robots.txt", root_url);
        if let Ok(resp) = self.client.get(&robots_url).send().await {
            if resp.status().is_success() {
                if let Ok(text) = resp.text().await {
                    let (p, u) = Self::parse_robots_txt(&root_url, &text);
                    findings.extend(p);
                    discovered_urls.extend(u);
                }
            }
        }

        // 2. Fetch sitemap.xml
        let sitemap_urls = [
            format!("{}/sitemap.xml", root_url),
            format!("{}/sitemap_index.xml", root_url),
        ];

        for sm_url in &sitemap_urls {
            if let Ok(resp) = self.client.get(sm_url).send().await {
                if resp.status().is_success() {
                    if let Ok(text) = resp.text().await {
                        let (p, u) = Self::parse_sitemap_xml(&root_url, &text);
                        findings.extend(p);
                        discovered_urls.extend(u);
                    }
                }
            }
        }

        (findings, discovered_urls)
    }

    pub fn parse_robots_txt(root_url: &str, content: &str) -> (Vec<(String, String, DiscoveredParam)>, HashSet<String>) {
        let mut findings = Vec::new();
        let mut urls = HashSet::new();

        for line in content.lines() {
            let trimmed = line.trim();
            let lower = trimmed.to_lowercase();
            if lower.starts_with("disallow:") || lower.starts_with("allow:") {
                let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let path = parts[1].trim();
                    if path.is_empty() || path == "/" {
                        continue;
                    }

                    let full_url = if path.starts_with('/') {
                        format!("{}{}", root_url, path)
                    } else {
                        format!("{}/{}", root_url, path)
                    };

                    urls.insert(full_url.clone());

                    if let Ok(parsed) = Url::parse(&full_url) {
                        let base = format!("{}://{}", parsed.scheme(), parsed.host_str().unwrap_or_default());
                        let p_path = parsed.path().to_string();

                        for (name, val) in parsed.query_pairs() {
                            if !name.trim().is_empty() {
                                findings.push((
                                    base.clone(),
                                    p_path.clone(),
                                    DiscoveredParam {
                                        name: name.to_string(),
                                        source: ParamSource::RobotsTxt,
                                        sample_value: if val.is_empty() { None } else { Some(val.to_string()) },
                                    },
                                ));
                            }
                        }
                    }
                }
            }
        }

        (findings, urls)
    }

    pub fn parse_sitemap_xml(_root_url: &str, content: &str) -> (Vec<(String, String, DiscoveredParam)>, HashSet<String>) {
        let mut findings = Vec::new();
        let mut urls = HashSet::new();

        let re = match Regex::new(r#"<loc>\s*(https?://[^<\s]+)\s*</loc>"#) {
            Ok(r) => r,
            Err(_) => return (findings, urls),
        };

        for cap in re.captures_iter(content) {
            if let Some(matched) = cap.get(1) {
                let loc_url = matched.as_str().trim();
                urls.insert(loc_url.to_string());

                if let Ok(parsed) = Url::parse(loc_url) {
                    let base = format!("{}://{}", parsed.scheme(), parsed.host_str().unwrap_or_default());
                    let p_path = parsed.path().to_string();

                    for (name, val) in parsed.query_pairs() {
                        if !name.trim().is_empty() {
                            findings.push((
                                base.clone(),
                                p_path.clone(),
                                DiscoveredParam {
                                    name: name.to_string(),
                                    source: ParamSource::Sitemap,
                                    sample_value: if val.is_empty() { None } else { Some(val.to_string()) },
                                },
                            ));
                        }
                    }
                }
            }
        }

        (findings, urls)
    }

    fn get_root_url(target: &str) -> Option<String> {
        let parsed = Url::parse(target).ok()?;
        Some(format!("{}://{}", parsed.scheme(), parsed.host_str()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_robots_txt() {
        let robots = r#"
            User-agent: *
            Disallow: /admin/dashboard?export=all&token=secret
            Allow: /search?q=public
        "#;
        let (findings, urls) = RobotsSitemapHarvester::parse_robots_txt("https://example.com", robots);
        assert_eq!(urls.len(), 2);
        let names: Vec<_> = findings.iter().map(|(_, _, p)| p.name.as_str()).collect();
        assert!(names.contains(&"export"));
        assert!(names.contains(&"token"));
        assert!(names.contains(&"q"));
        assert_eq!(findings[0].2.source, ParamSource::RobotsTxt);
    }

    #[test]
    fn test_parse_sitemap_xml() {
        let xml = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
               <url>
                  <loc>https://example.com/item?id=42&amp;ref=sitemap</loc>
               </url>
            </urlset>
        "#;
        let (findings, urls) = RobotsSitemapHarvester::parse_sitemap_xml("https://example.com", xml);
        assert_eq!(urls.len(), 1);
        let names: Vec<_> = findings.iter().map(|(_, _, p)| p.name.as_str()).collect();
        assert!(names.contains(&"id"));
        assert_eq!(findings[0].2.source, ParamSource::Sitemap);
    }
}
