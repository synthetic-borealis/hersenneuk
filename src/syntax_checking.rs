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
