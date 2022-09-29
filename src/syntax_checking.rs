pub fn is_valid_brainfuck(code: &str) -> bool {
    let instructions: Vec<char> = code.chars().collect();
    let loop_start_count = instructions.iter().filter(|&&c| c == '[').count();
    let loop_end_count = instructions.iter().filter(|&&c| c == ']').count();
    loop_start_count == loop_end_count
}
