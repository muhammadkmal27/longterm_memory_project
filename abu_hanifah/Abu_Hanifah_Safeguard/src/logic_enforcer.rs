use std::fs;
use colored::*;
use std::path::Path;
use crate::scanner::SecurityIssue;

pub fn run_logic_enforce(_url: &str, project_path: &Path) {
    println!("\n{}", "🧠 Memulakan Fasa 4: Deep Architectural & Business Logic Enforcer (White-Box)...".magenta().bold());
    println!("Menganalisis logik perniagaan di laluan: {:?}", project_path);

    let mut vulnerabilities_found = 0;
    let mut findings = Vec::new();

    let backend_src_path = project_path.join("backend/src");

    if !backend_src_path.exists() {
        println!("⚠️ Folder sumber backend tidak dijumpai di laluan yang diberikan.");
        return;
    }

    // Fungsi rekursif untuk membaca fail merentas 6 teknologi (Rust, PHP, TS, Java)
    fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&fs::DirEntry)) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if path.is_dir() && !["node_modules", "vendor", "target", ".git"].contains(&name.as_str()) {
                    visit_dirs(&path, cb)?;
                } else if path.is_file() {
                    let ext = path.extension().unwrap_or_default().to_string_lossy();
                    if ["rs", "php", "ts", "tsx", "java"].contains(&ext.as_ref()) {
                        cb(&entry);
                    }
                }
            }
        }
        Ok(())
    }

    let mut all_files = Vec::new();
    let _ = visit_dirs(&project_path, &mut |entry| {
        all_files.push(entry.path());
    });

    for file_path in all_files {
        if let Ok(content) = fs::read_to_string(&file_path) {
            let file_name = file_path.file_name().unwrap_or_default().to_string_lossy();

            // 1. Semakan IDOR Autopilot
            // Format yang biasa disalahguna: `WHERE id =` (Raw), `where('id', ...)` (Laravel), `.findOne({ id: ... })` (Nest/TypeORM)
            let has_raw_sql = content.contains("SELECT") && content.contains("WHERE id =");
            let has_orm_sql = content.contains("where('id'") || content.contains("where(\"id\"");
            let has_nosql = content.contains(".findOne({ id:") || content.contains(".findOne({id:");
            
            if has_raw_sql || has_orm_sql || has_nosql {
                // STRATEGI JARING LUAS: Tangkap semua carian by ID tanpa tapisan
                println!("  {} Potensi IDOR dijumpai di fail {}!", "🔴 CRITICAL:".red().bold(), file_name);
                vulnerabilities_found += 1;
                findings.push(SecurityIssue {
                    file_path: file_path.to_string_lossy().to_string(),
                    line_number: 1,
                    vulnerability_type: "Insecure Direct Object Reference (IDOR)".to_string(),
                    description: format!("Fail `{}` mengandungi carian pangkalan data spesifik (ID). Pastikan wujud penapis `user_id`.", file_name),
                    severity: "KRITIKAL (CRITICAL)".to_string(),
                });
            }
            
            // Mass Assignment (Heuristik)
            if content.contains("all()") || content.contains("req.body") {
                if content.contains("create(") || content.contains("update(") || content.contains("save(") {
                    println!("  {} Potensi Mass Assignment di fail {}!", "⚠️ WARNING:".yellow().bold(), file_name);
                    vulnerabilities_found += 1;
                    findings.push(SecurityIssue {
                        file_path: file_path.to_string_lossy().to_string(),
                        line_number: 1,
                        vulnerability_type: "Mass Assignment".to_string(),
                        description: format!("Fail `{}` menerima input proksi terus. Sahkan tiada Mass Assignment berlaku.", file_name),
                        severity: "TINGGI (HIGH)".to_string(),
                    });
                }
            }

            // 2. Semakan Race Condition (Ketiadaan Transaction / Mutex pada operasi UPDATE kritikal)
            let has_update = content.contains("UPDATE") || content.contains(".update(") || content.contains("->update(");
            let has_critical_field = content.contains("balance") || content.contains("credit");
            
            if has_update && has_critical_field {
                // STRATEGI JARING LUAS: Tangkap semua operasi update kredit secara membabi buta
                println!("  {} Potensi Race Condition dijumpai di fail {}!", "🔴 CRITICAL:".red().bold(), file_name);
                vulnerabilities_found += 1;
                findings.push(SecurityIssue {
                    file_path: file_path.to_string_lossy().to_string(),
                    line_number: 1,
                    vulnerability_type: "Race Condition (Concurrency Flaw)".to_string(),
                    description: format!("Fail `{}` mengubah kredit kritikal. Sahkan blok Transaksi pangkalan data wujud.", file_name),
                    severity: "TINGGI (HIGH)".to_string(),
                });
            }
        }
    }

    // 3. Semakan Middleware Auth pada Router
    let router_path = backend_src_path.join("router.rs");
    if let Ok(content) = fs::read_to_string(&router_path) {
        if content.contains("Router::new()") && !content.contains("layer(middleware::from_fn(require_auth))") {
            println!("  {} Ketiadaan Global Auth Middleware di router.rs!", "🔴 CRITICAL:".red().bold());
            vulnerabilities_found += 1;
            findings.push(SecurityIssue {
                file_path: router_path.to_string_lossy().to_string(),
                line_number: 1,
                vulnerability_type: "Broken Access Control".to_string(),
                description: "Tiada pelaksanaan `require_auth` middleware secara menyeluruh pada tetapan Router utama.".to_string(),
                severity: "KRITIKAL (CRITICAL)".to_string(),
            });
        }
    }

    if vulnerabilities_found > 0 {
        crate::reporter::print_report(&findings, project_path);
        println!("\n{}", "❌ Pelayan mempunyai lompang logik kritikal pada senibina kod! Laporan dikemaskini.".red().bold());
    } else {
        println!("\n{}", "✅ Ujian Senibina Logik selesai. Tiada kerentanan kritikal ditemui. Laporan dikemaskini.".green().bold());
    }
}
