use std::fs::File;
use std::io::{BufRead, BufReader};

/// Built-in high-confidence Bug Bounty Parameter Dictionary (Curated from HackerOne, Arjun & SecLists)
pub static TOP_BUG_BOUNTY_PARAMS: &[&str] = &[
    // Debug, Dev & Administrative
    "debug", "admin", "test", "dev", "preview", "enable", "disable", "view", "source", "show",
    "mode", "format", "json", "raw", "render", "dry_run", "internal", "beta", "version", "v",
    "trace", "log", "diag", "status", "health", "config", "env", "dump", "profiler", "sandbox",
    "mock", "fake", "bypass", "elevate", "su", "root", "sudo", "privilege", "impersonate", "as_user",
    
    // Redirect, SSRF & URL Fetching
    "redirect", "url", "next", "target", "dest", "destination", "return", "return_to", "rurl",
    "callback", "webhook", "feed", "host", "uri", "domain", "forward", "link", "goto", "out",
    "ref", "reference", "continue", "site", "html", "val", "validate", "fetch", "proxy", "request",
    "endpoint", "service", "api", "resource", "load_url", "remote", "server", "ip", "address", "port",
    "download", "read", "open", "browse", "source_url", "origin", "redirect_uri", "login_url", "logout_url",

    // Search, Query & XSS High-Reflection Candidates
    "q", "query", "search", "keyword", "term", "find", "filter", "sort", "order", "dir", "by",
    "category", "tag", "lang", "locale", "msg", "message", "error", "alert", "prompt", "title",
    "name", "content", "text", "comment", "description", "note", "body", "input", "value", "str",
    "headline", "subject", "notice", "warning", "info", "highlight", "caption", "label", "detail",
    "feedback", "question", "answer", "reason", "summary", "snippet", "display", "output", "alias",

    // Identity, Auth, OAuth & Tokens
    "id", "user", "user_id", "uid", "account", "account_id", "email", "token", "key", "api_key",
    "auth", "auth_token", "access_token", "refresh_token", "bearer", "code", "otp", "session",
    "session_id", "sess", "cookie", "jwt", "role", "group", "team", "org", "member", "profile",
    "access", "secret", "hash", "signature", "sig", "client_id", "client_secret", "state", "nonce",
    "pass", "password", "passwd", "pin", "credential", "private_key", "public_key", "certificate",

    // Pagination, Slicing & Views
    "page", "p", "limit", "offset", "size", "count", "per_page", "start", "end", "from", "to",
    "skip", "take", "max", "min", "cursor", "rows", "range", "step", "since", "until", "total",
    "num", "number", "index", "pos", "position", "chunk", "segment", "batch", "slice",

    // File Inclusion, Paths & Templates (LFI / RFI / SSTI)
    "file", "path", "doc", "document", "folder", "dir_path", "template", "include", "page_id",
    "module", "action", "cmd", "exec", "view_file", "load", "layout", "theme", "style", "css",
    "script", "attach", "attachment", "filename", "filepath", "report", "export", "import",
    "schema", "model", "view_model", "tpl", "renderer", "component", "widget", "partial",

    // Financial, Checkout & Shopping (Business Logic & Race Conditions)
    "amount", "price", "total_price", "currency", "qty", "quantity", "discount", "coupon",
    "promo", "voucher", "balance", "cart_id", "order_id", "item_id", "product_id", "payment_id",
    "tx_id", "transaction_id", "fee", "tax", "rate", "subtotal", "credit", "debit", "invoice_id",

    // Database & SQL Injection Common Parameters
    "tbl", "table", "col", "column", "field", "db", "database", "where", "group_by", "having",
    "union", "select", "insert", "update", "delete", "criteria", "condition", "statement",
];

pub fn get_default_wordlist() -> Vec<String> {
    TOP_BUG_BOUNTY_PARAMS
        .iter()
        .map(|s| s.to_string())
        .collect()
}

pub fn load_wordlist_from_file(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            words.push(trimmed.to_string());
        }
    }

    Ok(words)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordlist_not_empty() {
        let list = get_default_wordlist();
        assert!(list.len() > 100);
        assert!(list.contains(&"debug".to_string()));
        assert!(list.contains(&"redirect".to_string()));
        assert!(list.contains(&"q".to_string()));
        assert!(list.contains(&"client_id".to_string()));
        assert!(list.contains(&"price".to_string()));
    }

    #[test]
    fn test_load_wordlist_from_file() {
        let temp_path = std::env::temp_dir().join("test_ah_wordlist.txt");
        let sample = "custom_param_1\n# Comment\ncustom_param_2\n";
        std::fs::write(&temp_path, sample).unwrap();

        let loaded = load_wordlist_from_file(temp_path.to_str().unwrap()).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0], "custom_param_1");
        assert_eq!(loaded[1], "custom_param_2");

        let _ = std::fs::remove_file(temp_path);
    }
}
