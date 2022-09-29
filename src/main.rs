use hersenneuk::{code_cleanup, interpreter, syntax_checking};
use std::{env, fs, process};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let exe_name = args.remove(0).split('\\').last().unwrap().to_string();
    if args.is_empty() {
        print_usage(exe_name);
        println!("No input provided!");
        process::exit(-1);
    }
    if args.contains(&"help".to_string()) || args.contains(&"--help".to_string()) {
        print_usage(exe_name);
        process::exit(0);
    }

    let source_file = args.remove(0);
    let mut source_code = fs::read_to_string(source_file).expect("Could not read input file!");
    source_code = code_cleanup::clean_code(&code_cleanup::strip_comments(&source_code));

    if syntax_checking::is_valid_brainfuck(&source_code) {
        interpreter::run(&source_code);
    } else {
        println!("Oh noes :'(");
        process::exit(-1);
    }
}

fn print_usage(exe_name: String) {
    println!("Hersenneuk v0.0.1");
    println!("Usage: {} SOURCE-FILE", exe_name);
}
