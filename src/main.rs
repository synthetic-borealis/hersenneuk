use std::fs;
use hersenneuk::*;

fn main() {
    // let source_file_path = "hello_world.bf";
    let source_file_path = "hello_with_comments.bf";
    let mut hello_world_code = fs::read_to_string(source_file_path)
        .expect("Could not read input file!");
    hello_world_code = clean_code(&strip_comments(&hello_world_code));

    if is_valid_brainfuck(&hello_world_code) {
        run(&hello_world_code);
    } else {
        println!("Oh noes :'(");
    }
}
