use std::collections::VecDeque;
use std::io::{Read, stdin, stdout, Write};

const BLOCK_SIZE: usize = 30000;

pub fn run(code: String) {
    let instructions: Vec<char> = code.chars().collect();
    let loop_start_count = instructions.iter().filter(|&&c| c == '[').count();
    let loop_end_count = instructions.iter().filter(|&&c| c == ']').count();
    if loop_start_count != loop_end_count {
        println!("Error: Mismatching brackets detected");
        return;
    }

    let mut cursor: usize = 0;
    let mut cells: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
    let mut position: usize = 0;
    let mut loop_positions: VecDeque<usize> = VecDeque::new();

    while cursor < instructions.len() {
        match instructions[cursor] {
            '<' => if position > 0 {
                position -= 1;
            },
            '>' => if position < BLOCK_SIZE - 1 {
                position += 1;
            },
            '-' => if cells[position] > 0 {
                cells[position] -= 1;
            },
            '+' => if cells[position] < 255 {
                cells[position] += 1;
            },
            ',' => {
                cells[position] = get_char() as u8;
            },
            '.' => {
                put_char(cells[position] as char);
            },
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
            },
            ']' => if cells[position] != 0 {
                cursor = *loop_positions.back().unwrap();
            } else {
                loop_positions.pop_back();
            },
            _ => {},
        }
        cursor += 1;
    }
}

fn put_char(c: char) {
    let buf: [u8; 1] = [c as u8];
    stdout().write(&buf).unwrap();
}

fn get_char() -> char {
    let mut buf: [u8; 1] = [0];
    stdin().read_exact(&mut buf).unwrap();
    buf[0] as char
}
