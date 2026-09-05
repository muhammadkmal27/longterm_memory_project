use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "abu_hanifah_parameters",
    author = "Abu Hanifah AI Security <security@abuhanifah.ai>",
    version = "1.0.0",
    about = "High-Performance Web Parameter Discovery & Dalfox Formatter for Bug Bounty",
    long_about = "Abu_Hanifah_Parameters is a blazing-fast parameter discovery tool written in Rust.\n\
                  It combines Passive Archive Mining, Deep JavaScript AST/Regex Mining, and Heuristic\n\
                  Hidden Parameter Discovery to output Dalfox-ready injection URLs."
)]
pub struct Cli {
    /// Single target URL to inspect (e.g. https://target.com/page)
    #[arg(short = 'u', long = "url")]
    pub url: Option<String>,

    /// Target domain or subdomain from HackerOne scope (e.g. target.com)
    #[arg(short = 'd', long = "domain")]
    pub domain: Option<String>,

    /// File containing list of domains/subdomains (HackerOne eligible scope)
    #[arg(short = 'l', long = "list")]
    pub list: Option<String>,

    /// Format output specifically for Dalfox (URL with FUZZ placeholders ready for piping)
    #[arg(long = "dalfox")]
    pub dalfox: bool,

    /// Output individual URLs for each parameter separately instead of merging all into one URL
    #[arg(long = "split")]
    pub split: bool,

    /// Silent mode (suppress banners and progress logs, print only findings)
    #[arg(short = 's', long = "silent")]
    pub silent: bool,

    /// Output results in JSON format
    #[arg(long = "json")]
    pub json: bool,

    /// Save output to file
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,

    /// Number of concurrent requests
    #[arg(short = 'c', long = "concurrency", default_value = "20")]
    pub concurrency: usize,

    /// Request timeout in seconds
    #[arg(long = "timeout", default_value = "10")]
    pub timeout: u64,

    /// Custom User-Agent header
    #[arg(long = "user-agent")]
    pub user_agent: Option<String>,

    /// Custom HTTP headers (format: 'Header: Value', can be specified multiple times)
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    /// Disable passive archive mining (Wayback Machine & OTX)
    #[arg(long = "no-passive")]
    pub no_passive: bool,

    /// Disable client-side JavaScript bundle mining
    #[arg(long = "no-js")]
    pub no_js: bool,

    /// Disable active heuristic hidden-parameter mining
    #[arg(long = "no-heuristic")]
    pub no_heuristic: bool,

    /// Crawl depth for recursive spider (1-5, default: 2)
    #[arg(long = "depth", default_value = "2")]
    pub depth: usize,

    /// Maximum pages to crawl per domain (default: 50)
    #[arg(long = "max-pages", default_value = "50")]
    pub max_pages: usize,

    /// Custom wordlist file for active parameter discovery
    #[arg(short = 'w', long = "wordlist")]
    pub wordlist: Option<String>,

    /// HTTP method for active heuristic probing: get, post, or both (default: get)
    #[arg(long = "method", default_value = "get")]
    pub method: String,

    /// Disable recursive spider / page crawler
    #[arg(long = "no-spider")]
    pub no_spider: bool,

    /// Disable robots.txt and sitemap.xml harvesting
    #[arg(long = "no-robots")]
    pub no_robots: bool,

    /// Disable URLScan.io passive search
    #[arg(long = "no-urlscan")]
    pub no_urlscan: bool,

    /// Disable Common Crawl index search
    #[arg(long = "no-commoncrawl")]
    pub no_commoncrawl: bool,
}
