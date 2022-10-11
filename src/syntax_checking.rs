//! Miscellaneous syntax checking functions.
use regex::Regex;

/// Checks whether a program contains mismatching loop bounds.
pub fn has_mismatching_loop_bounds(code: &str) -> bool {
    let mut loop_count = 0;
    for c in code.chars() {
        if c == '[' {
            loop_count += 1;
        } else if c == ']' {
            loop_count -= 1;
        }
        if loop_count < 0 {
            return true;
        }
    }
    loop_count != 0
}

/// Checks whether a program contains potential infinite loops.
pub fn has_infinite_loops(code: &str) -> bool {
    let re = Regex::new(r"\[[+.]*]").unwrap();
    !matches!(re.find(code), None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const MISMATCHING_BOUND_SOURCE_FILE: &str = "test_assets/hello_mismatch.bf";
    const INFINITE_LOOP_SOURCE_FILE: &str = "test_assets/inf_loop.bf";

    #[test]
    fn it_detects_mismatching_loop_bounds() {
        let test_source = fs::read_to_string(MISMATCHING_BOUND_SOURCE_FILE).unwrap();
        assert!(has_mismatching_loop_bounds(&test_source));
    }

    #[test]
    fn it_detects_infinite_loops() {
        let test_source = fs::read_to_string(INFINITE_LOOP_SOURCE_FILE).unwrap();
        assert!(has_infinite_loops(&test_source));
    }
}
