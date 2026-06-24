use clap::Parser;
use colored::*;
use std::path::PathBuf;

mod reporter;
mod rules;
mod scanner;
mod taint_analyzer;
mod config_auditor;
mod logic_enforcer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Pilih fasa (1=SAST Asas, 2=SAST Lanjutan, 3=DAST, 4=AI Pentester)
    #[arg(long, default_value_t = 1)]
    phase: u8,

    /// Path to the directory to scan (Fasa 1 & 2)
    #[arg(long, default_value = ".")]
    path: PathBuf,

    /// URL to attack (Fasa 3 & 4)
    #[arg(long, default_value = "http://localhost:3000")]
    url: String,
}

fn main() {
    let args = Args::parse();
    
    println!("{}", "=======================================".cyan().bold());
    println!("{}", "🛡️  Abu Hanifah Safeguard: Edisi 4-Fasa".cyan().bold());
    println!("{}", "=======================================".cyan().bold());
    
    match args.phase {
        1 => {
            println!("Menjalankan Fasa 1: SAST Asas (AST Pattern Matching) di {}", args.path.display());
            let mut vulnerabilities_found = 0;
            match scanner::scan_directory(&args.path) {
                Ok(issues) => {
                    if issues.is_empty() {
                        println!("\n{}", "✅ Tiada kerentanan kritikal ditemui pada Fasa 1. Kod kelihatan selamat!".green().bold());
                    } else {
                        vulnerabilities_found = issues.len();
                        reporter::print_report(&issues, &args.path);
                    }
                }
                Err(e) => eprintln!("{} {}", "Ralat semasa mengimbas:".red().bold(), e),
            }
            if vulnerabilities_found > 0 { std::process::exit(1); }
        },
        2 => taint_analyzer::run_taint_analysis(&args.path),
        3 => config_auditor::run_audit(&args.url, &args.path),
        4 => logic_enforcer::run_logic_enforce(&args.url, &args.path),
        _ => {
            eprintln!("{} Fasa tidak disokong. Sila pilih Fasa 1, 2, 3, atau 4.", "❌".red());
            std::process::exit(1);
        }
    }
}
