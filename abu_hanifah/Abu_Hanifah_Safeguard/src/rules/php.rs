use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor};
use crate::scanner::SecurityIssue;

pub fn scan_file(path: &Path) -> Result<Vec<SecurityIssue>, std::io::Error> {
    let source_code = fs::read_to_string(path)?;
    let mut parser = Parser::new();
    
    parser.set_language(&tree_sitter_php::LANGUAGE_PHP.into()).expect("Ralat memuatkan nahu PHP");
    
    let tree = parser.parse(&source_code, None).unwrap();
    let root_node = tree.root_node();
    
    let mut issues = Vec::new();
    
    // AST Query 1a: Laravel Eloquent ORM Race Condition
    let race_query_source = r#"
        (method_call_expression 
            name: (name) @method_name
            (#match? @method_name "^(update|insert|save|decrement|increment)$")
        ) @update_call
    "#;
    
    if let Ok(race_query) = Query::new(&tree_sitter_php::LANGUAGE_PHP.into(), race_query_source) {
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&race_query, root_node, source_code.as_bytes());
        let has_transaction = source_code.contains("DB::transaction");
        if !has_transaction {
            for m in matches {
                for capture in m.captures {
                    if capture.index == 0 {
                        let node = capture.node;
                        issues.push(SecurityIssue {
                            file_path: path.to_string_lossy().to_string(),
                            line_number: node.start_position().row + 1,
                            vulnerability_type: "Race Condition (Laravel ORM)".to_string(),
                            description: "Kemaskini pangkalan data (update/insert) dikesan tanpa kehadiran blok `DB::transaction`. Ini terdedah kepada serangan pembayaran serentak.".to_string(),
                            severity: "KRITIKAL (CRITICAL)".to_string(),
                        });
                    }
                }
            }
        }
    }
    
    // AST Query 1b: PHP Native Race Condition / Raw SQLi Risk (OOP & Procedural)
    let raw_php_query_source = r#"
        (function_call_expression
            function: (name) @func_name
            (#match? @func_name "^(mysqli_query|mysql_query|pg_query)$")
        ) @db_call
        
        (member_call_expression
            name: (name) @method_name
            (#match? @method_name "^(query|execute)$")
        ) @db_call
    "#;
    
    if let Ok(raw_query) = Query::new(&tree_sitter_php::LANGUAGE_PHP.into(), raw_php_query_source) {
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&raw_query, root_node, source_code.as_bytes());
        let has_begin = source_code.to_lowercase().contains("begin transaction") || source_code.to_lowercase().contains("start transaction");
        
        // Dapatkan index untuk @db_call
        if let Some(db_call_index) = raw_query.capture_index_for_name("db_call") {
            for m in matches {
                for capture in m.captures {
                    if capture.index == db_call_index {
                        let node = capture.node;
                        let text = node.utf8_text(source_code.as_bytes()).unwrap_or("");
                        let upper_text = text.to_uppercase();
                        
                        // Semak jika query ini melaksanakan manipulasi wang/stok tanpa transaksi
                        if (upper_text.contains("UPDATE ") || upper_text.contains("INSERT ")) && !has_begin {
                            issues.push(SecurityIssue {
                                file_path: path.to_string_lossy().to_string(),
                                line_number: node.start_position().row + 1,
                                vulnerability_type: "Race Condition (PHP Native/OOP)".to_string(),
                                description: "Operasi SQL mengandungi UPDATE/INSERT tetapi tiada blok BEGIN TRANSACTION ditemui. Sangat terdedah kepada *Race Condition*.".to_string(),
                                severity: "KRITIKAL (CRITICAL)".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
    
    // AST Query 2: Mengesan echo tanpa escape atau fungsi bahaya (XSS)
    // Untuk Laravel Blade, tree-sitter php tidak parse blade tag sepenuhnya, 
    // tetapi kita boleh fallback ke pattern matching jika AST PHP tulen digunakan.
    // Di sini kita tangkap fungsi `echo` pada AST PHP tulen.
    let echo_query_source = r#"
        (echo_statement) @echo_stmt
    "#;
    
    if let Ok(echo_query) = Query::new(&tree_sitter_php::LANGUAGE_PHP.into(), echo_query_source) {
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&echo_query, root_node, source_code.as_bytes());
        
        for m in matches {
            for capture in m.captures {
                let node = capture.node;
                let text = node.utf8_text(source_code.as_bytes()).unwrap_or("").to_lowercase();
                
                // STRATEGI JARING LUAS (High Sensitivity)
                // Tangkap secara membuta tuli sebarang echo yang mempunyai pembolehubah,
                // tanpa mempedulikan kehadiran fungsi sanitasi (Antigravity akan menapisnya kelak).
                let mut is_vulnerable = false;
                if text.contains("$") {
                    is_vulnerable = true;
                }
                    
                if is_vulnerable {
                    issues.push(SecurityIssue {
                        file_path: path.to_string_lossy().to_string(),
                        line_number: node.start_position().row + 1,
                        vulnerability_type: "XSS (Cross-Site Scripting)".to_string(),
                        description: "Penyataan `echo` pembolehubah secara langsung tanpa sanitasi dikesan (seperti `htmlspecialchars`).".to_string(),
                        severity: "TINGGI (HIGH)".to_string(),
                    });
                }
            }
        }
    }
    
    // AST Query 3: Mengesan SQL Injection (Penyambungan String / Interpolation dalam query)
    let sqli_php_query_source = r#"
        [
            (function_call_expression
                function: (name) @func_name
                (#match? @func_name "^(mysqli_query|mysql_query|pg_query)$")
                arguments: (arguments
                    [
                        (binary_expression operator: ".")
                        (encapsed_string)
                    ]
                )
            ) @sqli_call
            
            (member_call_expression
                name: (name) @method_name
                (#match? @method_name "^(query|execute|raw|select)$")
                arguments: (arguments
                    [
                        (binary_expression operator: ".")
                        (encapsed_string)
                    ]
                )
            ) @sqli_call
            
            (scoped_call_expression
                name: (name) @scoped_name
                (#match? @scoped_name "^(raw|select|statement)$")
                arguments: (arguments
                    [
                        (binary_expression operator: ".")
                        (encapsed_string)
                    ]
                )
            ) @sqli_call
            
            (function_call_expression
                function: (name) @func_name
                (#match? @func_name "^(mysqli_query|mysql_query|pg_query)$")
                arguments: (arguments
                    (encapsed_string)
                )
            ) @sqli_call
            
            (member_call_expression
                name: (name) @method_name
                (#match? @method_name "^(query|execute|raw|select)$")
                arguments: (arguments
                    (encapsed_string)
                )
            ) @sqli_call
        ]
    "#;
    
    if let Ok(sqli_query) = Query::new(&tree_sitter_php::LANGUAGE_PHP.into(), sqli_php_query_source) {
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&sqli_query, root_node, source_code.as_bytes());
        
        if let Some(sqli_call_index) = sqli_query.capture_index_for_name("sqli_call") {
            for m in matches {
                for capture in m.captures {
                    if capture.index == sqli_call_index {
                        let node = capture.node;
                        issues.push(SecurityIssue {
                            file_path: path.to_string_lossy().to_string(),
                            line_number: node.start_position().row + 1,
                            vulnerability_type: "SQL Injection (SQLi)".to_string(),
                            description: "Penyambungan rentetan (string concatenation '.') atau *variable interpolation* dikesan di dalam fungsi pangkalan data. Gunakan *Prepared Statements* atau *Parameterized Queries* (contoh: PDO binding atau Laravel Eloquent bindings) bagi mengelak SQL Injection.".to_string(),
                            severity: "KRITIKAL (CRITICAL)".to_string(),
                        });
                    }
                }
            }
        }
    }
    // AST Query 4-8: Gabungan Kerentanan Baharu (Secrets, CmdInj, LFI, Rand, Redirect)
    let extra_php_query_source = r#"
        (assignment_expression
            left: (variable_name (name) @var_name)
            (#match? @var_name "(?i)(secret|password|token|key)")
            right: (string)
        ) @secret_call
        
        (function_call_expression
            function: (name) @func_name
            (#match? @func_name "^(system|exec|shell_exec|passthru|popen)$")
        ) @cmd_call
        
        (function_call_expression
            function: (name) @func_name
            (#match? @func_name "^(file_get_contents|fopen|readfile)$")
            arguments: (arguments [ (binary_expression) (variable_name) (encapsed_string) ])
        ) @lfi_call
        
        (function_call_expression
            function: (name) @func_name
            (#match? @func_name "^(rand|mt_rand|lcg_value)$")
        ) @rand_call
        
        (function_call_expression
            function: (name) @func_name
            (#eq? @func_name "header")
            arguments: (arguments [ (binary_expression) (encapsed_string) (variable_name) ])
        ) @redirect_call
    "#;
    
    if let Ok(extra_query) = Query::new(&tree_sitter_php::LANGUAGE_PHP.into(), extra_php_query_source) {
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&extra_query, root_node, source_code.as_bytes());
        
        let secret_idx = extra_query.capture_index_for_name("secret_call");
        let cmd_idx = extra_query.capture_index_for_name("cmd_call");
        let lfi_idx = extra_query.capture_index_for_name("lfi_call");
        let rand_idx = extra_query.capture_index_for_name("rand_call");
        let redirect_idx = extra_query.capture_index_for_name("redirect_call");
        
        for m in matches {
            for capture in m.captures {
                let node = capture.node;
                let line_num = node.start_position().row + 1;
                let path_str = path.to_string_lossy().to_string();
                
                if Some(capture.index) == secret_idx {
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Hardcoded Secrets".to_string(), description: "Pembolehubah berunsur rahsia/kata laluan dikesan berformat *plaintext*. Gunakan `env()`.".to_string(), severity: "KRITIKAL (CRITICAL)".to_string() });
                } else if Some(capture.index) == cmd_idx {
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Command Injection / RCE".to_string(), description: "Penggunaan fungsi perlaksanaan OS (seperti `system` / `exec`). Sangat berisiko Remote Code Execution (RCE).".to_string(), severity: "KRITIKAL (CRITICAL)".to_string() });
                } else if Some(capture.index) == lfi_idx {
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Directory Traversal / LFI".to_string(), description: "Fungsi pembacaan fail statik disuap dengan nilai dinamik. Boleh menyebabkan pengekstrakan fail dalaman.".to_string(), severity: "TINGGI (HIGH)".to_string() });
                } else if Some(capture.index) == rand_idx {
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Insecure Randomness".to_string(), description: "Penggunaan nombor rawak tidak selamat secara kriptografi (`rand/mt_rand`). Gunakan `random_bytes()`.".to_string(), severity: "SEDERHANA (MEDIUM)".to_string() });
                } else if Some(capture.index) == redirect_idx {
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Open Redirect".to_string(), description: "Penggunaan `header()` dengan nilai dinamik dikesan. Sahkan url untuk mengelak lencongan ke domain palsu.".to_string(), severity: "TINGGI (HIGH)".to_string() });
                }
            }
        }
    }
    
    Ok(issues)
}
