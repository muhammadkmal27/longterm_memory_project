use std::fs;
use colored::*;
use std::path::Path;
use crate::scanner::SecurityIssue;

pub fn run_audit(_url: &str, project_path: &Path) {
    println!("\n{}", "🚀 Memulakan Fasa 3: Semantic Dependency & Configuration Auditor (White-Box)...".magenta().bold());
    println!("Menganalisis persekitaran di laluan: {:?}", project_path);
    
    let mut vulnerabilities_found = 0;
    let mut findings = Vec::new();

    // 1. Check CORS Misconfiguration in router.rs or equivalent
    let router_path = project_path.join("backend/src/router.rs");
    if let Ok(content) = fs::read_to_string(&router_path) {
        if content.contains("AllowOrigin::any()") || content.contains("Access-Control-Allow-Origin: *") {
            println!("  {} Terjumpa konfigurasi CORS Wildcard (*) di router.rs!", "🔴 CRITICAL:".red().bold());
            vulnerabilities_found += 1;
            findings.push(SecurityIssue {
                file_path: router_path.to_string_lossy().to_string(),
                line_number: 1,
                vulnerability_type: "CORS Misconfiguration (White-box)".to_string(),
                description: "Fail `router.rs` dikesan menggunakan `AllowOrigin::any()`. Ini mendedahkan pelayan kepada serangan rentas-domain.".to_string(),
                severity: "KRITIKAL (CRITICAL)".to_string(),
            });
        } else {
            println!("  {} Konfigurasi CORS disahkan selamat.", "✅".green());
        }
    } else {
        println!("  {} Fail `router.rs` tidak ditemui, semakan CORS dilangkau.", "⚠️".yellow());
    }

    // 2. Check for Hardcoded Secrets in .env or config files
    let env_path = project_path.join(".env");
    let gitignore_path = project_path.join(".gitignore");
    
    if env_path.exists() {
        // STRATEGI JARING LUAS: Tangkap fail .env tanpa mengira .gitignore
        println!("  {} Fail `.env` dikesan. Sahkan ia tidak mengandungi rahsia produksi.", "⚠️ WARNING:".yellow().bold());
        vulnerabilities_found += 1;
        findings.push(SecurityIssue {
                file_path: env_path.to_string_lossy().to_string(),
                line_number: 1,
                vulnerability_type: "API Key / Secret Leaks".to_string(),
                description: "Fail `.env` wujud di laluan utama. Sila pastikan tiada maklumat rahsia terdedah.".to_string(),
                severity: "TINGGI (HIGH)".to_string(),
        });
    }
    
    // Semakan Reverse Proxy Misconfiguration
    let nginx_path = project_path.join("nginx.conf");
    if nginx_path.exists() {
        println!("  {} Konfigurasi Nginx dikesan. Sila semak Reverse Proxy Misconfigurations.", "⚠️ WARNING:".yellow().bold());
        vulnerabilities_found += 1;
        findings.push(SecurityIssue {
                file_path: nginx_path.to_string_lossy().to_string(),
                line_number: 1,
                vulnerability_type: "Reverse Proxy Misconfigurations".to_string(),
                description: "Fail `nginx.conf` ditemui. Periksa tetapan `proxy_pass` dan `X-Forwarded-For`.".to_string(),
                severity: "SEDERHANA (MEDIUM)".to_string(),
        });
    }

    // 3. Check Cargo.toml for outdated/vulnerable typical dependencies
    let cargo_path = project_path.join("backend/Cargo.toml");
    if let Ok(content) = fs::read_to_string(&cargo_path) {
        if content.contains("md5 =") {
            println!("  {} Algoritma lapuk MD5 dikesan di `Cargo.toml`!", "🔴 CRITICAL:".red().bold());
            vulnerabilities_found += 1;
            findings.push(SecurityIssue {
                file_path: cargo_path.to_string_lossy().to_string(),
                line_number: 1,
                vulnerability_type: "Cryptographic Failures (MD5)".to_string(),
                description: "Kebergantungan pustaka `md5` dikesan. Sila tukar kepada Bcrypt atau Argon2.".to_string(),
                severity: "KRITIKAL (CRITICAL)".to_string(),
            });
        } else {
            println!("  {} Semakan pustaka selamat (Tiada modul kriptografi lapuk dikesan).", "✅".green());
        }
    }

    if vulnerabilities_found > 0 {
        crate::reporter::print_report(&findings, project_path);
        println!("\n{}", "❌ Pelayan mempunyai masalah keselamatan infrastruktur! Laporan dikemaskini.".red().bold());
    } else {
        println!("\n{}", "✅ Ujian Pengaudit Konfigurasi selesai. Tiada kelumpuhan dikesan. Laporan dikemaskini.".green().bold());
    }
}
