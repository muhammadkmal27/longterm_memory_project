mod active;
mod aggregator;
mod cli;
mod crawler;
mod models;
mod passive;
mod reporter;
mod scanner;

use aggregator::ParamAggregator;
use clap::Parser;
use cli::Cli;
use colored::Colorize;
use models::ScanOptions;
use reporter::CliReporter;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use scanner::ParamScanner;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let is_dalfox_piped = args.dalfox || args.silent;
    if !is_dalfox_piped {
        CliReporter::print_banner();
    }

    // 1. Gather all targets
    let mut targets = Vec::new();
    if let Some(ref u) = args.url {
        targets.push(u.clone());
    }
    if let Some(ref d) = args.domain {
        targets.push(d.clone());
    }
    if let Some(ref list_path) = args.list {
        match File::open(list_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines().flatten() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() && !trimmed.starts_with('#') {
                        targets.push(trimmed.to_string());
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "{} Gagal membaca fail senarai {}: {}",
                    "[ERROR]".bright_red().bold(),
                    list_path.bright_white(),
                    e
                );
                std::process::exit(1);
            }
        }
    }

    if targets.is_empty() {
        eprintln!(
            "{} Sila nyatakan sasaran menggunakan -u <URL>, -d <DOMAIN>, atau -l <SENARAI_HACKERONE>",
            "[!]".bright_yellow().bold()
        );
        eprintln!("    Contoh: abu_hanifah_parameters -u https://target.com/search --dalfox");
        std::process::exit(1);
    }

    // 2. Build HTTP Client
    let mut headers = HeaderMap::new();
    for h in &args.headers {
        if let Some((k, v)) = h.split_once(':') {
            if let (Ok(name), Ok(val)) = (
                HeaderName::from_bytes(k.trim().as_bytes()),
                HeaderValue::from_str(v.trim()),
            ) {
                headers.insert(name, val);
            }
        }
    }

    let default_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AbuHanifahParameters/1.0";
    let user_agent = args.user_agent.as_deref().unwrap_or(default_ua);

    let client = match Client::builder()
        .user_agent(user_agent)
        .default_headers(headers)
        .timeout(Duration::from_secs(args.timeout))
        .redirect(reqwest::redirect::Policy::limited(5))
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} Gagal membina HTTP client: {}", "[ERROR]".bright_red(), e);
            std::process::exit(1);
        }
    };

    let scan_opts = ScanOptions {
        concurrency: args.concurrency,
        timeout_secs: args.timeout,
        user_agent: user_agent.to_string(),
        dalfox_mode: args.dalfox,
        silent: args.silent,
        enable_passive: !args.no_passive,
        enable_urlscan: !args.no_urlscan,
        enable_commoncrawl: !args.no_commoncrawl,
        enable_js: !args.no_js,
        enable_spider: !args.no_spider,
        enable_robots: !args.no_robots,
        enable_heuristic: !args.no_heuristic,
        depth: args.depth,
        max_pages: args.max_pages,
        wordlist_path: args.wordlist,
        method: args.method,
    };

    let scanner = ParamScanner::new(client, scan_opts);
    let mut all_raw_findings = Vec::new();

    for target in &targets {
        let findings = scanner.scan_target(target).await;
        all_raw_findings.extend(findings);
    }

    // 3. Aggregate & Normalize
    let aggregated = ParamAggregator::aggregate(all_raw_findings);
    let total_unique_params: usize = aggregated.iter().map(|ep| ep.params.len()).sum();

    // 4. Output Results
    if args.dalfox {
        CliReporter::print_dalfox_urls(&aggregated, args.split);
    } else if args.json {
        let json_str = serde_json::to_string_pretty(&aggregated).unwrap_or_default();
        println!("{}", json_str);
    } else if !args.silent {
        CliReporter::print_summary(&aggregated, total_unique_params);
    } else {
        CliReporter::print_dalfox_urls(&aggregated, args.split);
    }

    // 5. Save to output file if requested
    if let Some(ref out_path) = args.output {
        let res = if args.json {
            CliReporter::save_json(&aggregated, out_path)
        } else {
            CliReporter::save_to_file(&aggregated, out_path, args.split)
        };

        if let Err(e) = res {
            eprintln!("{} Gagal menyimpan output ke fail: {}", "[ERROR]".bright_red(), e);
        } else if !is_dalfox_piped {
            println!(
                " {} Hasil berjaya disimpan ke: {}",
                "[SAVED]".bright_green().bold(),
                out_path.bright_white()
            );
        }
    }
}
