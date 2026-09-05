use crate::active::{get_default_wordlist, load_wordlist_from_file, HeuristicMiner, ReflectionTester};
use crate::crawler::{HtmlParser, JsMiner, RobotsSitemapHarvester, SpiderEngine};
use crate::models::{DiscoveredParam, ScanOptions};
use crate::passive::{CommonCrawlMiner, OtxMiner, UrlScanMiner, WaybackMiner};
use colored::Colorize;
use reqwest::Client;
use std::collections::HashSet;
use std::sync::Arc;
use url::Url;

pub struct ParamScanner {
    client: Client,
    options: ScanOptions,
}

impl ParamScanner {
    pub fn new(client: Client, options: ScanOptions) -> Self {
        Self { client, options }
    }

    pub async fn scan_target(&self, target: &str) -> Vec<(String, String, DiscoveredParam)> {
        let mut all_findings = Vec::new();
        let domain = Self::extract_domain(target);
        let target_url = Self::ensure_schema(target);

        if !self.options.silent && !self.options.dalfox_mode {
            println!("{} Memulakan imbasan: {}", "[*]".bright_cyan(), target.bright_white().bold());
        }

        // 1. Passive OSINT Mining (Wayback, OTX, URLScan, CommonCrawl)
        if self.options.enable_passive {
            let passive_findings = self.scan_passive(&domain).await;
            if !self.options.silent && !self.options.dalfox_mode && !passive_findings.is_empty() {
                println!(" {} [Passive OSINT] Ditemui {} parameter arkib", "[+]".bright_green(), passive_findings.len().to_string().bright_yellow());
            }
            all_findings.extend(passive_findings);
        }

        // 2. Robots.txt & Sitemap.xml Harvester
        if self.options.enable_robots {
            let (robot_params, _) = RobotsSitemapHarvester::new(self.client.clone()).harvest(&target_url).await;
            if !self.options.silent && !self.options.dalfox_mode && !robot_params.is_empty() {
                println!(" {} [Robots/Sitemap] Ditemui {} parameter fail konfigurasi", "[+]".bright_green(), robot_params.len().to_string().bright_yellow());
            }
            all_findings.extend(robot_params);
        }

        // 3. Web Crawler & Spider (HTML, Forms, Inline JS, External Scripts)
        let (crawl_params, js_script_urls) = self.scan_crawler(&target_url).await;
        if !self.options.silent && !self.options.dalfox_mode && !crawl_params.is_empty() {
            println!(" {} [Crawler/Spider] Ditemui {} parameter dari halaman web", "[+]".bright_green(), crawl_params.len().to_string().bright_yellow());
        }
        all_findings.extend(crawl_params);

        // 4. Client-Side External JavaScript Mining
        if self.options.enable_js && !js_script_urls.is_empty() {
            let js_params = self.scan_external_js(&target_url, js_script_urls).await;
            if !self.options.silent && !self.options.dalfox_mode && !js_params.is_empty() {
                println!(" {} [JS Bundles] Ditemui {} parameter tersembunyi JS", "[+]".bright_green(), js_params.len().to_string().bright_yellow());
            }
            all_findings.extend(js_params);
        }

        // 5. Active Heuristic & Canary Reflection (XSS Hunter)
        if self.options.enable_heuristic {
            let active_findings = self.scan_active(&target_url).await;
            all_findings.extend(active_findings);
        }

        all_findings
    }

    async fn scan_passive(&self, domain: &str) -> Vec<(String, String, DiscoveredParam)> {
        let mut results = Vec::new();
        let wb = WaybackMiner::new(self.client.clone());
        if let Ok(urls) = wb.fetch_urls(domain).await {
            results.extend(WaybackMiner::extract_params_from_urls(&urls));
        }
        let otx = OtxMiner::new(self.client.clone());
        if let Ok(urls) = otx.fetch_urls(domain).await {
            results.extend(OtxMiner::extract_params_from_urls(&urls));
        }
        if self.options.enable_urlscan {
            let us = UrlScanMiner::new(self.client.clone());
            if let Ok(urls) = us.fetch_urls(domain).await {
                results.extend(UrlScanMiner::extract_params_from_urls(&urls));
            }
        }
        if self.options.enable_commoncrawl {
            let cc = CommonCrawlMiner::new(self.client.clone());
            if let Ok(urls) = cc.fetch_urls(domain).await {
                results.extend(CommonCrawlMiner::extract_params_from_urls(&urls));
            }
        }
        results
    }

    async fn scan_crawler(&self, target_url: &str) -> (Vec<(String, String, DiscoveredParam)>, HashSet<String>) {
        let mut params = Vec::new();
        let mut js_script_urls = HashSet::new();
        let js_miner = JsMiner::new();

        if self.options.enable_spider {
            let spider = SpiderEngine::new(self.client.clone(), self.options.depth, self.options.max_pages);
            let s_res = spider.crawl(target_url).await;
            params.extend(s_res.params);
            js_script_urls.extend(s_res.js_script_urls);
            for (base, path, script_text) in s_res.inline_scripts {
                params.extend(js_miner.extract_params_from_inline_js(&base, &path, &script_text));
            }
        } else if let Ok(resp) = self.client.get(target_url).send().await {
            if resp.status().is_success() {
                if let Ok(body) = resp.text().await {
                    let page = HtmlParser::parse_html(target_url, &body);
                    params.extend(page.params);
                    js_script_urls.extend(page.js_script_urls);
                    let (b, p) = Self::split_url(target_url);
                    for inline in page.inline_scripts {
                        params.extend(js_miner.extract_params_from_inline_js(&b, &p, &inline));
                    }
                }
            }
        }

        (params, js_script_urls)
    }

    async fn scan_external_js(&self, base_url: &str, js_urls: HashSet<String>) -> Vec<(String, String, DiscoveredParam)> {
        let js_miner = Arc::new(JsMiner::new());
        let mut js_tasks = Vec::new();

        for js_url in js_urls.into_iter().take(30) {
            let client = self.client.clone();
            let miner = Arc::clone(&js_miner);
            let base = base_url.to_string();

            js_tasks.push(tokio::spawn(async move {
                if let Ok(resp) = client.get(&js_url).send().await {
                    if let Ok(js_text) = resp.text().await {
                        return miner.extract_params_from_js(&base, "/", &js_text);
                    }
                }
                Vec::new()
            }));
        }

        let mut results = Vec::new();
        for task in js_tasks {
            if let Ok(p) = task.await {
                results.extend(p);
            }
        }
        results
    }

    async fn scan_active(&self, target_url: &str) -> Vec<(String, String, DiscoveredParam)> {
        let mut results = Vec::new();
        let (base, path) = Self::split_url(target_url);
        let wordlist = if let Some(ref w_path) = self.options.wordlist_path {
            load_wordlist_from_file(w_path).unwrap_or_else(|_| get_default_wordlist())
        } else {
            get_default_wordlist()
        };

        // Canary Reflection Hunter
        let reflector = ReflectionTester::new(self.client.clone());
        let reflected = reflector.check_batch_reflection(target_url, &wordlist, &self.options.method).await;
        if !self.options.silent && !self.options.dalfox_mode && !reflected.is_empty() {
            println!(" {} [Reflection Hunter] Ditemui {} parameter terpantul (High XSS Risk)!", "[!]".bright_red().bold(), reflected.len().to_string().bright_yellow().bold());
        }
        for p in reflected {
            results.push((base.clone(), path.clone(), p));
        }

        // Dichotomy Heuristic
        let heuristic = HeuristicMiner::new(self.client.clone(), 25, self.options.method.clone());
        let hidden = heuristic.discover_hidden_params(target_url, &wordlist).await;
        if !self.options.silent && !self.options.dalfox_mode && !hidden.is_empty() {
            println!(" {} [Heuristic Engine] Ditemui {} hidden parameter aktif!", "[+]".bright_green().bold(), hidden.len().to_string().bright_cyan().bold());
        }
        for p in hidden {
            results.push((base.clone(), path.clone(), p));
        }

        results
    }

    fn extract_domain(target: &str) -> String {
        if let Ok(parsed) = Url::parse(target) {
            return parsed.host_str().unwrap_or(target).to_string();
        }
        let clean = target.trim_start_matches("https://").trim_start_matches("http://");
        clean.split('/').next().unwrap_or(clean).to_string()
    }

    fn ensure_schema(target: &str) -> String {
        if target.starts_with("http://") || target.starts_with("https://") {
            target.to_string()
        } else {
            format!("https://{}", target)
        }
    }

    fn split_url(target: &str) -> (String, String) {
        if let Ok(parsed) = Url::parse(target) {
            let base = format!("{}://{}", parsed.scheme(), parsed.host_str().unwrap_or_default());
            (base, parsed.path().to_string())
        } else {
            (target.to_string(), "/".to_string())
        }
    }
}
