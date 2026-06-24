use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SecurityIssue {
    pub file_path: String,
    pub line_number: usize,
    pub vulnerability_type: String,
    pub description: String,
    pub severity: String,
}

pub fn scan_directory(path: &Path) -> Result<Vec<SecurityIssue>, std::io::Error> {
    let mut all_issues = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| {
        let name = e.file_name().to_string_lossy();
        let ignored_dirs = [
            // JavaScript/TypeScript (Next.js, Nest.js, React)
            "node_modules", ".next", "out", "dist", "build", "coverage",
            // PHP (Laravel, Native)
            "vendor", "storage", "phpmyadmin",
            // Rust (Axum)
            "target",
            // Java (Spring Boot)
            ".gradle", ".mvn",
            // Sistem & IDE
            ".git", ".idea", ".vscode"
        ];
        !ignored_dirs.contains(&name.as_ref())
    }).filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                let ext_str = extension.to_string_lossy();
                
                // Scan TS/TSX files for React/Next.js vulnerabilities
                if ext_str == "ts" || ext_str == "tsx" || ext_str == "js" || ext_str == "jsx" {
                    if let Ok(issues) = crate::rules::tsx::scan_file(path) {
                        all_issues.extend(issues);
                    }
                }
                
                // Scan PHP files for Laravel vulnerabilities
                if ext_str == "php" {
                    if let Ok(issues) = crate::rules::php::scan_file(path) {
                        all_issues.extend(issues);
                    }
                }
                
                // Scan Rust files for Axum vulnerabilities
                if ext_str == "rs" {
                    if let Ok(issues) = crate::rules::rust::scan_file(path) {
                        all_issues.extend(issues);
                    }
                }
                
                // Scan Java files for Spring Boot vulnerabilities
                if ext_str == "java" {
                    if let Ok(issues) = crate::rules::java::scan_file(path) {
                        all_issues.extend(issues);
                    }
                }
            }
        }
    }
    
    Ok(all_issues)
}
