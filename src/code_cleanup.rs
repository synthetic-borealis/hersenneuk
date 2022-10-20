//! Code cleanup functions.
use regex::Regex;

/// Strips C++-style single-line & multi-line comments comments.
pub fn strip_comments(code: &str) -> String {
    let multi_line_comment = Regex::new(r"/[*]([^*]|([*][^/]))*[*]/").unwrap();
    let single_line_comment = Regex::new(r"/{2,}[^\r^\n]*").unwrap();
    let first_pass = multi_line_comment.replace_all(code, "").into_owned();
    single_line_comment
        .replace_all(&first_pass, "")
        .into_owned()
}

/// Removes characters that aren't valid Brainfuck instructions.
pub fn clean_code(code: &str) -> String {
    let re = Regex::new(r"[^<>+\-,.\[\]]*").unwrap();
    re.replace_all(code, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const SOURCE_FILE_WITH_COMMENTS: &str = "test_assets/hello_with_comments.bf";
    const SOURCE_FILE_WITHOUT_COMMENTS: &str = "test_assets/hello_world.bf";

    #[test]
    fn it_cleans_source_correctly() {
        let control_source = fs::read_to_string(SOURCE_FILE_WITHOUT_COMMENTS).unwrap();
        let mut test_source = fs::read_to_string(SOURCE_FILE_WITH_COMMENTS).unwrap();
        test_source = clean_code(&strip_comments(&test_source));
        assert_eq!(control_source, test_source);
    }
}
