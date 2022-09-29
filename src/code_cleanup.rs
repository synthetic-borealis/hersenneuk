use regex::Regex;

pub fn strip_comments(code: &str) -> String {
    let re = Regex::new(r"/{2,}[^\r^\n]*").unwrap();
    re.replace_all(code, "").to_string()
}

pub fn clean_code(code: &str) -> String {
    let re = Regex::new(r"[^<>+\-,.\[\]]*").unwrap();
    re.replace_all(code, "").to_string()
}
