use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor};
use crate::scanner::SecurityIssue;

pub fn scan_file(path: &Path) -> Result<Vec<SecurityIssue>, std::io::Error> {
    let source_code = fs::read_to_string(path)?;
    let mut parser = Parser::new();
    
    parser.set_language(&tree_sitter_java::LANGUAGE.into()).expect("Ralat memuatkan nahu Java");
    
    let tree = parser.parse(&source_code, None).unwrap();
    let root_node = tree.root_node();
    
    let mut issues = Vec::new();
    
    // STRATEGI JARING LUAS (High Sensitivity): Pengimbasan Teks Kasar untuk Java
    let lines: Vec<&str> = source_code.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.contains("ObjectInputStream") {
            issues.push(SecurityIssue {
                file_path: path.to_string_lossy().to_string(),
                line_number: i + 1,
                vulnerability_type: "Insecure Deserialization".to_string(),
                description: "Penggunaan `ObjectInputStream` dikesan. Boleh dieksploit untuk Remote Code Execution.".to_string(),
                severity: "KRITIKAL (CRITICAL)".to_string(),
            });
        }
        if line.contains("@CrossOrigin") {
            issues.push(SecurityIssue {
                file_path: path.to_string_lossy().to_string(),
                line_number: i + 1,
                vulnerability_type: "CORS Misconfiguration".to_string(),
                description: "Penggunaan `@CrossOrigin` dikesan. Sahkan adakah ia membenarkan asalan luaran yang berbahaya.".to_string(),
                severity: "TINGGI (HIGH)".to_string(),
            });
        }
        if line.contains("@GetMapping") || line.contains("@PostMapping") || line.contains("@RequestMapping") {
            if !source_code.contains("@PreAuthorize") && !source_code.contains("SecurityConfig") {
                issues.push(SecurityIssue {
                    file_path: path.to_string_lossy().to_string(),
                    line_number: i + 1,
                    vulnerability_type: "Broken Access Control".to_string(),
                    description: "Endpoint dikesan tetapi tiada tanda amaran `@PreAuthorize`. Sahkan kawalan akses berpusat.".to_string(),
                    severity: "TINGGI (HIGH)".to_string(),
                });
            }
        }
    }
    
    // AST Query: Detect string concatenation in EntityManager.createQuery() or jdbcTemplate.query()
    let java_sqli_source = r#"
        (method_invocation
            name: (identifier) @method_name
            (#match? @method_name "^(createQuery|query)$")
            arguments: (argument_list
                (binary_expression
                    operator: "+"
                )
            )
        )
    "#;
    
    if let Ok(query) = Query::new(&tree_sitter_java::LANGUAGE.into(), java_sqli_source) {
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
                        description: "Cantuman string (+) di dalam panggilan query pangkalan data dikesan. Sangat berisiko untuk HQL/SQL Injection. Gunakan Prepared Statements atau Named Parameters.".to_string(),
                        severity: "KRITIKAL (CRITICAL)".to_string(),
                    });
                }
            }
        }
    }
    // AST Query 2-6: Gabungan Kerentanan Baharu untuk Java Spring Boot
    let extra_java_query_source = r#"
        (local_variable_declaration
            declarator: (variable_declarator
                name: (identifier) @var_name
                (#match? @var_name "(?i)(secret|password|token|key)")
                value: (string_literal)
            )
        ) @secret_call
        
        (method_invocation
            name: (identifier) @func_name
            (#eq? @func_name "exec")
        ) @cmd_call
        
        (method_invocation
            name: (identifier) @func_name
            (#match? @func_name "^(readAllBytes|readAllLines)$")
        ) @lfi_call
        
        (object_creation_expression
            type: (type_identifier) @type_name
            (#eq? @type_name "Random")
        ) @rand_call
        
        (method_invocation
            name: (identifier) @func_name
            (#eq? @func_name "sendRedirect")
        ) @redirect_call
    "#;
    
    if let Ok(extra_query) = Query::new(&tree_sitter_java::LANGUAGE.into(), extra_java_query_source) {
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
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Hardcoded Secrets".to_string(), description: "Pembolehubah rahsia dikesan tertulis terus ke dalam fail kod Java. Guna `application.properties`.".to_string(), severity: "KRITIKAL (CRITICAL)".to_string() });
                } else if Some(capture.index) == cmd_idx {
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Command Injection / RCE".to_string(), description: "Pemanggilan arahan OS secara langsung dikesan. Sangat berbahaya.".to_string(), severity: "KRITIKAL (CRITICAL)".to_string() });
                } else if Some(capture.index) == lfi_idx {
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Directory Traversal / LFI".to_string(), description: "Pembacaan fail `readAllBytes` boleh digodam jika disuap parameter manipulasi.".to_string(), severity: "TINGGI (HIGH)".to_string() });
                } else if Some(capture.index) == rand_idx {
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Insecure Randomness".to_string(), description: "Gunakan `SecureRandom` dan bukannya kelas `Random` biasa.".to_string(), severity: "SEDERHANA (MEDIUM)".to_string() });
                } else if Some(capture.index) == redirect_idx {
                    issues.push(SecurityIssue { file_path: path_str.clone(), line_number: line_num, vulnerability_type: "Open Redirect".to_string(), description: "Pelencongan pengguna menerusi `sendRedirect()` boleh dilarikan jika parameter URL tidak diuji.".to_string(), severity: "TINGGI (HIGH)".to_string() });
                }
            }
        }
    }
    
    Ok(issues)
}
