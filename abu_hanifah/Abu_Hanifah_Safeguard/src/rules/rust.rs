use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor};
use crate::scanner::SecurityIssue;

pub fn scan_file(path: &Path) -> Result<Vec<SecurityIssue>, std::io::Error> {
    let source_code = fs::read_to_string(path)?;
    let mut parser = Parser::new();
    
    // We assume tree-sitter-rust has LANGUAGE
    parser.set_language(&tree_sitter_rust::LANGUAGE.into()).expect("Ralat memuatkan nahu Rust");
    
    let tree = parser.parse(&source_code, None).unwrap();
    let root_node = tree.root_node();
    
    let mut issues = Vec::new();
    
    // STRATEGI JARING LUAS (High Sensitivity): Pengimbasan Teks Kasar untuk Rust
    let lines: Vec<&str> = source_code.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.contains(".unwrap()") || line.contains(".expect(") {
            issues.push(SecurityIssue {
                file_path: path.to_string_lossy().to_string(),
                line_number: i + 1,
                vulnerability_type: "Mishandling of Exceptional Conditions".to_string(),
                description: "Penggunaan `.unwrap()` atau `.expect()` dikesan. Sangat berisiko *Panic* (Denial of Service).".to_string(),
                severity: "TINGGI (HIGH)".to_string(),
            });
        }
        if line.contains("unsafe {") {
            issues.push(SecurityIssue {
                file_path: path.to_string_lossy().to_string(),
                line_number: i + 1,
                vulnerability_type: "Memory Corruption / Unsafe".to_string(),
                description: "Blok `unsafe` dikesan. Membuka ruang kepada *Memory Corruption*.".to_string(),
                severity: "KRITIKAL (CRITICAL)".to_string(),
            });
        }
    }
    
    // AST Query 1: Detect raw string concatenation inside SQLx queries (SQLi Risk)
    // Looking for format!("SELECT * FROM users WHERE name = '{}'", input) inside sqlx::query!
    let sqli_query_source = r#"
        (macro_invocation
            macro: (identifier) @macro_name
            (#match? @macro_name "^query!?$")
            (token_tree
                (macro_invocation
                    macro: (identifier) @format
                    (#eq? @format "format")
                )
            )
        )
    "#;
    
    if let Ok(query) = Query::new(&tree_sitter_rust::LANGUAGE.into(), sqli_query_source) {
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&query, root_node, source_code.as_bytes());
        for m in matches {
            for capture in m.captures {
                if capture.index == 0 {
                    let node = capture.node;
                    issues.push(SecurityIssue {
                        file_path: path.to_string_lossy().to_string(),
                        line_number: node.start_position().row + 1,
                        vulnerability_type: "SQL Injection (SQLi)".to_string(),
                        description: "Penggunaan `format!` di dalam fungsi query SQL dikesan. Ini boleh membawa kepada SQL Injection jika input tidak dibersihkan. Gunakan `sqlx` query arguments (`$1`, `?`) sebaliknya.".to_string(),
                        severity: "KRITIKAL (CRITICAL)".to_string(),
                    });
                }
            }
        }
    }
    // AST Query 2-5: Gabungan Kerentanan Baharu untuk Rust
    let extra_rust_query_source = r#"
        (let_declaration
            pattern: (identifier) @var_name
            (#match? @var_name "(?i)(secret|password|token|key)")
            value: (string_literal)
        ) @secret_call
        
        (call_expression
            function: (scoped_identifier
                path: (identifier) @path_name
                name: (identifier) @func_name
            )
            (#eq? @path_name "Command")
            (#eq? @func_name "new")
        ) @cmd_call
        
        (call_expression
            function: (scoped_identifier
                name: (identifier) @func_name
            )
            (#eq? @func_name "read_to_string")
        ) @lfi_call
        
        (call_expression
            function: (scoped_identifier
                path: (identifier) @path_name
                name: (identifier) @func_name
            )
            (#eq? @path_name "Redirect")
            (#eq? @func_name "to")
        ) @redirect_call
    "#;
    
    if let Ok(extra_query) = Query::new(&tree_sitter_rust::LANGUAGE.into(), extra_rust_query_source) {
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&extra_query, root_node, source_code.as_bytes());
        
        let secret_idx = extra_query.capture_index_for_name("secret_call");
        let cmd_idx = extra_query.capture_index_for_name("cmd_call");
        let lfi_idx = extra_query.capture_index_for_name("lfi_call");
        let redirect_idx = extra_query.capture_index_for_name("redirect_call");
        
        for m in matches {
            for capture in m.captures {
                let node = capture.node;
                let line_num = node.start_position().row + 1;
                let path_str = path.to_string_lossy().to_string();
                
                // Dapatkan teks baris kod tersebut untuk semakan "Context-Aware"
                let start_byte = node.start_position().row;
                let source_lines: Vec<&str> = source_code.lines().collect();
                let code_line = if start_byte < source_lines.len() { source_lines[start_byte] } else { "" };
                
                if Some(capture.index) == secret_idx {
                    // Abaikan fail ujian untuk Hardcoded Secrets
                    if !path_str.ends_with("_test.rs") && !path_str.ends_with("_tests.rs") && !path_str.contains("test") {
                        issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Hardcoded Secrets".to_string(), description: "Pembolehubah rahsia dikesan dalam bentuk *plaintext*. Gunakan modul rahsia / `.env`.".to_string(), severity: "KRITIKAL (CRITICAL)".to_string() });
                    }
                } else if Some(capture.index) == cmd_idx {
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Command Injection / RCE".to_string(), description: "Pemanggilan arahan OS `Command::new` dikesan. Sahkan input pengguna untuk mengelak eksekusi terlarang.".to_string(), severity: "KRITIKAL (CRITICAL)".to_string() });
                } else if Some(capture.index) == lfi_idx {
                    // STRATEGI JARING LUAS: Tangkap semua tanpa menapis statik path
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Directory Traversal / LFI".to_string(), description: "Pembacaan fail (`read_to_string`) dikesan.".to_string(), severity: "TINGGI (HIGH)".to_string() });
                } else if Some(capture.index) == redirect_idx {
                    // STRATEGI JARING LUAS: Tangkap semua redirect
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Open Redirect".to_string(), description: "Respons `Redirect::to` Axum dikesan.".to_string(), severity: "TINGGI (HIGH)".to_string() });
                }
            }
        }
    }
    
    Ok(issues)
}
