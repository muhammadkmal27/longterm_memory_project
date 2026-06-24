use colored::*;
use crate::scanner::SecurityIssue;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn print_report(issues: &[SecurityIssue], target_path: &Path) {
    println!("\n{}", "⚠️  Laporan Audit Keselamatan Statik (SAST)".yellow().bold());
    println!("{}", "--------------------------------------------------".yellow());
    
    for (i, issue) in issues.iter().enumerate() {
        println!("{}. [{}] {}", i + 1, issue.severity.red().bold(), issue.vulnerability_type.yellow().bold());
        println!("   {} {}", "Fail:".cyan(), issue.file_path);
        println!("   {} {}", "Baris:".cyan(), issue.line_number);
        println!("   {} {}", "Huraian:".cyan(), issue.description);
        println!();
    }
    
    // Append to raw_safeguard_output.json
    let audit_file_path = target_path.join("raw_safeguard_output.json");
    
    let mut existing_issues: Vec<SecurityIssue> = Vec::new();
    if audit_file_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&audit_file_path) {
            if let Ok(parsed) = serde_json::from_str(&content) {
                existing_issues = parsed;
            }
        }
    }
    
    existing_issues.extend_from_slice(issues);
    
    if let Ok(mut file) = File::create(&audit_file_path) {
        let json_output = serde_json::to_string_pretty(&existing_issues).unwrap_or_else(|_| "[]".to_string());
        writeln!(file, "{}", json_output).unwrap();
        
        println!("\n{} Laporan JSON mentah telah ditambah ke {}", "💾".green(), audit_file_path.display());
    } else {
        println!("\n{} Gagal menyimpan fail raw_safeguard_output.json", "❌".red());
    }
}
