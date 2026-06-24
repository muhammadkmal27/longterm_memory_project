use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor};
use crate::scanner::SecurityIssue;

pub fn scan_file(path: &Path) -> Result<Vec<SecurityIssue>, std::io::Error> {
    let source_code = fs::read_to_string(path)?;
    let mut parser = Parser::new();
    
    // Configure parser for TSX
    parser.set_language(&tree_sitter_typescript::LANGUAGE_TSX.into()).expect("Ralat memuatkan nahu TSX");
    
    let tree = parser.parse(&source_code, None).unwrap();
    let root_node = tree.root_node();
    
    let mut issues = Vec::new();
    
    // AST Query untuk mencari atribut JSX `dangerouslySetInnerHTML`
    let query_source = r#"
        (jsx_attribute 
            (property_identifier) @prop_name
            (#eq? @prop_name "dangerouslySetInnerHTML")
        )
    "#;
    
    let query = Query::new(&tree_sitter_typescript::LANGUAGE_TSX.into(), query_source).expect("Query error");
    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, root_node, source_code.as_bytes());
    
    for m in matches {
        for capture in m.captures {
            let node = capture.node;
            let start_byte = node.start_position().row;
            let source_lines: Vec<&str> = source_code.lines().collect();
            let code_line = if start_byte < source_lines.len() { source_lines[start_byte] } else { "" };
            
            // STRATEGI JARING LUAS (High Sensitivity)
            // Tangkap secara membuta tuli dangerouslySetInnerHTML tanpa menapis static strings.
            
            issues.push(SecurityIssue {
                file_path: path.to_string_lossy().to_string(),
                line_number: node.start_position().row + 1,
                vulnerability_type: "XSS (Cross-Site Scripting)".to_string(),
                description: "Penggunaan 'dangerouslySetInnerHTML' dikesan dalam komponen React/Next.js. Ini boleh membenarkan suntikan kod klien yang berbahaya.".to_string(),
                severity: "TINGGI (HIGH)".to_string(),
            });
        }
    }
    // AST Query 2: Mengesan SQL Injection (Prisma / TypeORM / Raw SQL)
    let sqli_tsx_query_source = r#"
        (call_expression
            function: (member_expression property: (property_identifier) @method_name)
            (#match? @method_name "^(query|queryRawUnsafe|execute)$")
            arguments: (arguments
                [
                    (binary_expression operator: "+")
                    (template_string)
                ]
            )
        ) @sqli_call
    "#;
    
    if let Ok(sqli_query) = Query::new(&tree_sitter_typescript::LANGUAGE_TSX.into(), sqli_tsx_query_source) {
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
                            description: "Penggunaan templat rentetan (Template literals) atau penyambungan string (+) dikesan di dalam query pangkalan data (Prisma/TypeORM). Ini boleh mengakibatkan SQL Injection. Gunakan parameterized queries seperti `prisma.$queryRaw`.".to_string(),
                            severity: "KRITIKAL (CRITICAL)".to_string(),
                        });
                    }
                }
            }
        }
    }
    // AST Query 3-7: Gabungan Kerentanan Baharu (Secrets, CmdInj, LFI, Rand, Redirect)
    let extra_tsx_query_source = r#"
        (variable_declarator 
            name: (identifier) @var_name 
            (#match? @var_name "(?i)(secret|password|token|key)")
            value: (string)
        ) @secret_call
        
        (call_expression
            function: (identifier) @func_name
            (#match? @func_name "^(exec|execSync|spawn|spawnSync)$")
        ) @cmd_call

        (call_expression
            function: (member_expression property: (property_identifier) @func_name)
            (#match? @func_name "^(exec|execSync|spawn|spawnSync)$")
        ) @cmd_call
        
        (call_expression
            function: (member_expression property: (property_identifier) @func_name)
            (#match? @func_name "^(readFile|readFileSync)$")
            arguments: (arguments [ (binary_expression) (template_string) (identifier) ])
        ) @lfi_call
        
        (call_expression
            function: (member_expression object: (identifier) @obj property: (property_identifier) @prop)
            (#eq? @obj "Math")
            (#eq? @prop "random")
        ) @rand_call
        
        (call_expression
            function: (member_expression property: (property_identifier) @func_name)
            (#eq? @func_name "redirect")
        ) @redirect_call
    "#;
    
    if let Ok(extra_query) = Query::new(&tree_sitter_typescript::LANGUAGE_TSX.into(), extra_tsx_query_source) {
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
                
                let start_byte = node.start_position().row;
                let source_lines: Vec<&str> = source_code.lines().collect();
                let code_line = if start_byte < source_lines.len() { source_lines[start_byte] } else { "" };
                
                if Some(capture.index) == secret_idx {
                    if !path_str.ends_with(".spec.ts") && !path_str.ends_with(".test.ts") && !path_str.ends_with(".spec.tsx") && !path_str.contains("test") {
                        issues.push(SecurityIssue { file_path: path_str, line_number: line_num, vulnerability_type: "Hardcoded Secrets".to_string(), description: "Pembolehubah berunsur rahsia/kata laluan ditugaskan secara terus (*hardcoded*). Gunakan `process.env`.".to_string(), severity: "KRITIKAL (CRITICAL)".to_string() });
                    }
                } else if Some(capture.index) == cmd_idx {
                    issues.push(SecurityIssue { file_path: path_str, line_number: line_num, vulnerability_type: "Command Injection / RCE".to_string(), description: "Penggunaan fungsi OS seperti `exec` atau `spawn` dikesan. Sangat berisiko Remote Code Execution (RCE).".to_string(), severity: "KRITIKAL (CRITICAL)".to_string() });
                } else if Some(capture.index) == lfi_idx {
                    issues.push(SecurityIssue { file_path: path_str, line_number: line_num, vulnerability_type: "Directory Traversal / LFI".to_string(), description: "Pembacaan fail secara terus (`fs.readFileSync`) menggunakan nilai dinamik dikesan.".to_string(), severity: "TINGGI (HIGH)".to_string() });
                } else if Some(capture.index) == rand_idx {
                    issues.push(SecurityIssue { file_path: path_str, line_number: line_num, vulnerability_type: "Insecure Randomness".to_string(), description: "Penggunaan `Math.random()` tidak selamat secara kriptografi. Gunakan `crypto.randomBytes()`.".to_string(), severity: "SEDERHANA (MEDIUM)".to_string() });
                } else if Some(capture.index) == redirect_idx {
                    // STRATEGI JARING LUAS: Tangkap semua redirect secara membabi buta
                    issues.push(SecurityIssue { file_path: path_str, line_number: line_num, vulnerability_type: "Open Redirect".to_string(), description: "Pelencongan arah (*redirect*) klien dikesan. Sila pastikan parameter disemak untuk mengelak Open Redirect.".to_string(), severity: "TINGGI (HIGH)".to_string() });
                }
            }
        }
    }
    
    Ok(issues)
}
