use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParamSource {
    Wayback,
    Otx,
    UrlScan,
    CommonCrawl,
    HtmlInput,
    HtmlHidden,
    JsRegex,
    InlineJs,
    RobotsTxt,
    Sitemap,
    Heuristic,
    QueryString,
    Reflection,
}

impl std::fmt::Display for ParamSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParamSource::Wayback => write!(f, "Wayback"),
            ParamSource::Otx => write!(f, "OTX"),
            ParamSource::UrlScan => write!(f, "URLScan"),
            ParamSource::CommonCrawl => write!(f, "CommonCrawl"),
            ParamSource::HtmlInput => write!(f, "HTML:Input"),
            ParamSource::HtmlHidden => write!(f, "HTML:Hidden"),
            ParamSource::JsRegex => write!(f, "JS:Regex"),
            ParamSource::InlineJs => write!(f, "JS:Inline"),
            ParamSource::RobotsTxt => write!(f, "Robots.txt"),
            ParamSource::Sitemap => write!(f, "Sitemap.xml"),
            ParamSource::Heuristic => write!(f, "Heuristic"),
            ParamSource::QueryString => write!(f, "QueryString"),
            ParamSource::Reflection => write!(f, "XSS:Reflection"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredParam {
    pub name: String,
    pub source: ParamSource,
    pub sample_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedEndpoint {
    pub base_url: String,
    pub path: String,
    pub params: HashSet<String>,
}

impl AggregatedEndpoint {
    #[allow(dead_code)]
    pub fn new(base_url: String, path: String) -> Self {
        Self {
            base_url,
            path,
            params: HashSet::new(),
        }
    }

    pub fn to_dalfox_url(&self) -> String {
        if self.params.is_empty() {
            return format!("{}{}", self.base_url, self.path);
        }

        let mut sorted_params: Vec<_> = self.params.iter().collect();
        sorted_params.sort();

        let query_part = sorted_params
            .iter()
            .map(|param| format!("{}=FUZZ", param))
            .collect::<Vec<_>>()
            .join("&");

        let separator = if self.path.contains('?') { "&" } else { "?" };
        format!("{}{}{}{}", self.base_url, self.path, separator, query_part)
    }

    pub fn to_dalfox_urls_split(&self) -> Vec<String> {
        let mut sorted_params: Vec<_> = self.params.iter().collect();
        sorted_params.sort();

        let separator = if self.path.contains('?') { "&" } else { "?" };
        sorted_params
            .into_iter()
            .map(|param| format!("{}{}{}{}=FUZZ", self.base_url, self.path, separator, param))
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScanOptions {
    pub concurrency: usize,
    pub timeout_secs: u64,
    pub user_agent: String,
    pub dalfox_mode: bool,
    pub silent: bool,
    pub enable_passive: bool,
    pub enable_urlscan: bool,
    pub enable_commoncrawl: bool,
    pub enable_js: bool,
    pub enable_spider: bool,
    pub enable_robots: bool,
    pub enable_heuristic: bool,
    pub depth: usize,
    pub max_pages: usize,
    pub wordlist_path: Option<String>,
    pub method: String,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            concurrency: 20,
            timeout_secs: 10,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AbuHanifahParameters/2.0".to_string(),
            dalfox_mode: false,
            silent: false,
            enable_passive: true,
            enable_urlscan: true,
            enable_commoncrawl: true,
            enable_js: true,
            enable_spider: true,
            enable_robots: true,
            enable_heuristic: true,
            depth: 2,
            max_pages: 50,
            wordlist_path: None,
            method: "get".to_string(),
        }
    }
}
