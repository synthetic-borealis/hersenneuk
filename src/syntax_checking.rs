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
    false
}
