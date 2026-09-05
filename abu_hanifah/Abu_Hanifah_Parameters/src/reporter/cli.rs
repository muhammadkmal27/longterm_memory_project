use crate::models::AggregatedEndpoint;
use colored::Colorize;
use std::fs::File;
use std::io::Write;

pub struct CliReporter;

impl CliReporter {
    pub fn print_banner() {
        let banner = r#"
   ___   _           _  _             _  __      _     ___                               _               
  / _ \ | |__  _  _ | || | __ _ _ _  (_)| |_   __ _ | _ \ __ _  _ _  __ _  _ __   ___  | |_  ___  _ _  ___
 / /_\ \| '_ \| || || __ |/ _` | ' \ | || ' \ / _` ||  _// _` || '_|/ _` || '  \ / -_) |  _|/ -_)| '_|(_-<
/_/   \_|_.__/ \_,_||_||_|\__,_|_||_||_||_||_|\__,_||_|  \__,_||_|  \__,_||_|_|_|\___|  \__|\___||_|  /__/
        "#;
        println!("{}", banner.bright_cyan().bold());
        println!(
            "{}",
            "   [+] Abu_Hanifah_Parameters v2.0.0 | High-Performance Bug Bounty Parameter Miner"
                .bright_yellow()
        );
        println!(
            "{}",
            "   [+] Supercharged Engine: Spider (BFS), Passive OSINT (Wayback/OTX/URLScan/CommonCrawl)\n   [+] Active Fuzzing: 1,000+ Wordlist & Canary Reflection (XSS Hunter) | Dalfox Ready\n"
                .bright_black()
        );
    }

    pub fn print_dalfox_urls(endpoints: &[AggregatedEndpoint], split: bool) {
        for ep in endpoints {
            if !ep.params.is_empty() {
                if split {
                    for url in ep.to_dalfox_urls_split() {
                        println!("{}", url);
                    }
                } else {
                    println!("{}", ep.to_dalfox_url());
                }
            }
        }
    }

    pub fn print_summary(endpoints: &[AggregatedEndpoint], total_discovered_params: usize) {
        println!("{}", "==================================================================".bright_blue());
        println!("{}", "                     IMBASAN PARAMETER SELESAI                    ".bright_green().bold());
        println!("{}", "==================================================================".bright_blue());
        println!(
            " {} Jumlah Endpoint Unik    : {}",
            "[*]".bright_cyan(),
            endpoints.len().to_string().bright_white().bold()
        );
        println!(
            " {} Jumlah Parameter Unik   : {}",
            "[*]".bright_cyan(),
            total_discovered_params.to_string().bright_yellow().bold()
        );
        println!();

        println!("{}", "--- [ SENARAI PARAMETER MENGIK ENDPOINT ] ---".bright_magenta().bold());
        for ep in endpoints {
            if !ep.params.is_empty() {
                let mut sorted_params: Vec<_> = ep.params.iter().cloned().collect();
                sorted_params.sort();
                println!(
                    " {} {}{} ({} parameter):",
                    "[+]".bright_green(),
                    ep.base_url.bright_white(),
                    ep.path.bright_white(),
                    sorted_params.len().to_string().bright_yellow()
                );
                println!("     {}", sorted_params.join(", ").bright_cyan());
            }
        }
        println!();

        println!("{}", "--- [ URL BERSERTA INJECTION TOKEN (FUZZ) ] ---".bright_magenta().bold());
        for ep in endpoints {
            if !ep.params.is_empty() {
                println!("{} {}", "->".bright_green(), ep.to_dalfox_url().bright_white());
            }
        }
        println!();
        println!(
            "{} Anda boleh salurkan terus ke dalfox: {}",
            "[TIP]".bright_yellow().bold(),
            "abu_hanifah_parameters -u <URL> --dalfox | dalfox pipe".bright_cyan()
        );
    }

    pub fn save_to_file(endpoints: &[AggregatedEndpoint], path: &str, split: bool) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        for ep in endpoints {
            if !ep.params.is_empty() {
                if split {
                    for url in ep.to_dalfox_urls_split() {
                        writeln!(file, "{}", url)?;
                    }
                } else {
                    writeln!(file, "{}", ep.to_dalfox_url())?;
                }
            }
        }
        Ok(())
    }

    pub fn save_json(endpoints: &[AggregatedEndpoint], path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        let json_data = serde_json::to_string_pretty(endpoints)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        writeln!(file, "{}", json_data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dalfox_url_formatting() {
        let mut ep = AggregatedEndpoint::new("https://example.com".to_string(), "/api".to_string());
        ep.params.insert("id".to_string());
        ep.params.insert("token".to_string());

        let url = ep.to_dalfox_url();
        assert!(url.contains("id=FUZZ"));
        assert!(url.contains("token=FUZZ"));
        assert!(url.starts_with("https://example.com/api?"));
    }
}
