//! Brainfuck interpreter functions.
use std::collections::vec_deque::VecDeque;
use std::io::{Read, Write};

/// Runs Brainfuck code with a fixed-size block.
///
/// Note: This function does *NOT* perform any syntax checking.
pub fn run_with_fixed_block(
    code: &str,
    stdin: &mut impl Read,
    stdout: &mut impl Write,
    block_size: usize,
) {
    let instructions: Vec<char> = code.chars().collect();
    let mut cursor: usize = 0;
    let mut cells: Vec<u8> = vec![0; block_size];
    let mut position: usize = 0;
    let mut loop_positions: VecDeque<usize> = VecDeque::new();

    while cursor < instructions.len() {
        match instructions[cursor] {
            '<' => {
                if position > 0 {
                    position -= 1;
                }
            }
            '>' => {
                if position < block_size - 1 {
                    position += 1;
                }
            }
            '-' => {
                if cells[position] > 0 {
                    cells[position] -= 1;
                }
            }
            '+' => {
                if cells[position] < 255 {
                    cells[position] += 1;
                }
            }
            ',' => {
                cells[position] = get_char(stdin) as u8;
            }
            '.' => {
                put_char(cells[position] as char, stdout);
            }
            '[' => {
                if cells[position] != 0 {
                    loop_positions.push_back(cursor);
                } else {
                    cursor += 1;
                    let mut ignore_brace: usize = 1;
                    while cursor < instructions.len() - 1 && ignore_brace != 0 {
                        if instructions[cursor] == ']' {
                            ignore_brace -= 1;
                        } else if instructions[cursor] == '[' {
                            ignore_brace += 1;
                        }
                        cursor += 1;
                    }
                    cursor -= 1;
                }
            }
            ']' => {
                if cells[position] != 0 {
                    cursor = *loop_positions.back().unwrap();
                } else {
                    loop_positions.pop_back();
                }
            }
            _ => {}
        }
        cursor += 1;
    }
}

/// Runs Brainfuck code with a dynamic-sized block.
///
/// Note: This function does *NOT* perform any syntax checking.
pub fn run_with_dynamic_block(code: &str, stdin: &mut impl Read, stdout: &mut impl Write) {
    let instructions: Vec<char> = code.chars().collect();
    let mut cursor: usize = 0;
    let mut cells: Vec<u8> = vec![0];
    let mut position: usize = 0;
    let mut loop_positions: VecDeque<usize> = VecDeque::new();

    while cursor < instructions.len() {
        match instructions[cursor] {
            '<' => {
                if position > 0 {
                    position -= 1;
                }
            }
            '>' => {
                if position + 1 == cells.len() {
                    cells.push(0);
                }
                position += 1;
            }
            '-' => {
                if cells[position] > 0 {
                    cells[position] -= 1;
                }
            }
            '+' => {
                if cells[position] < 255 {
                    cells[position] += 1;
                }
            }
            ',' => {
                cells[position] = get_char(stdin) as u8;
            }
            '.' => {
                put_char(cells[position] as char, stdout);
            }
            '[' => {
                if cells[position] != 0 {
                    loop_positions.push_back(cursor);
                } else {
                    cursor += 1;
                    let mut ignore_brace: usize = 1;
                    while cursor < instructions.len() - 1 && ignore_brace != 0 {
                        if instructions[cursor] == ']' {
                            ignore_brace -= 1;
                        } else if instructions[cursor] == '[' {
                            ignore_brace += 1;
                        }
                        cursor += 1;
                    }
                    cursor -= 1;
                }
            }
            ']' => {
                if cells[position] != 0 {
                    cursor = *loop_positions.back().unwrap();
                } else {
                    loop_positions.pop_back();
                }
            }
            _ => {}
        }
        cursor += 1;
    }
}

fn put_char(c: char, stdout: &mut impl Write) {
    let buf: [u8; 1] = [c as u8];
    stdout.write_all(&buf).unwrap();
}

fn get_char(stdin: &mut impl Read) -> char {
    let mut buf: [u8; 1] = [0];
    stdin.read_exact(&mut buf).unwrap();
    buf[0] as char
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};
    use std::{fs, io};

    const HELLO_SOURCE_FILE: &str = "test_assets/hello_world.bf";
    const USER_INPUT_SOURCE_FILE: &str = "test_assets/user_input.bf";
    const BLOCK_SIZE: usize = 30000;
    const BUF_SIZE: usize = 1024 * 8;

    struct MockInput {
        buffer: Vec<u8>,
    }

    impl MockInput {
        pub fn from_str(s: &str) -> Self {
            Self {
                buffer: Vec::from(s),
            }
        }
    }

    impl Read for MockInput {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let mut r = Cursor::new(&self.buffer);
            r.read(buf)
        }
    }

    #[test]
    fn hello_world_fixed_block() {
        let test_source = fs::read_to_string(HELLO_SOURCE_FILE).unwrap();
        let mut stdout: Vec<u8> = Vec::new();
        let mut stdin = BufReader::with_capacity(BUF_SIZE, io::stdin());

        run_with_fixed_block(&test_source, &mut stdin, &mut stdout, BLOCK_SIZE);
        let stdout = String::from_utf8_lossy(stdout.as_slice()).to_string();

        assert_eq!(stdout.trim(), "Hello World!");
    }

    #[test]
    fn hello_world_dynamic_block() {
        let test_source = fs::read_to_string(HELLO_SOURCE_FILE).unwrap();
        let mut stdout: Vec<u8> = Vec::new();
        let mut stdin = BufReader::with_capacity(BUF_SIZE, io::stdin());

        run_with_dynamic_block(&test_source, &mut stdin, &mut stdout);
        let stdout = String::from_utf8_lossy(stdout.as_slice()).to_string();

        assert_eq!(stdout.trim(), "Hello World!");
    }

    #[test]
    fn user_input() {
        let test_source = fs::read_to_string(USER_INPUT_SOURCE_FILE).unwrap();
        let mut stdout: Vec<u8> = Vec::new();
        let mut stdin = MockInput::from_str("c\n");

        run_with_fixed_block(&test_source, &mut stdin, &mut stdout, BLOCK_SIZE);
        let stdout = String::from_utf8_lossy(stdout.as_slice()).to_string();

        assert_eq!(stdout.trim(), "c");
    }
}
