use clap::Parser;
use hersenneuk::{code_cleanup, interpreter, syntax_checking};
use std::io::{BufReader, BufWriter};
use std::{fs, io, process};

const BLOCK_SIZE: usize = 30000;
const BUF_SIZE: usize = 1024 * 8;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input source file
    input_file: String,

    /// Use a dynamic cells block
    #[arg(short = 'D', long, default_value_t = false)]
    with_dynamic_block: bool,
}

fn main() {
    let args = Args::parse();

    if !file_exists(&args.input_file) {
        println!("Error: can't read file {}", args.input_file);
        process::exit(-1);
    }

    let mut stdout = BufWriter::with_capacity(BUF_SIZE, io::stdout());
    let mut stdin = BufReader::with_capacity(BUF_SIZE, io::stdin());
    let mut source_code = fs::read_to_string(args.input_file).expect("Could not read input file!");
    source_code = code_cleanup::clean_code(&code_cleanup::strip_comments(&source_code));

    if syntax_checking::has_mismatching_loop_bounds(&source_code) {
        println!("Error: Mismatching loop bounds detected!");
        process::exit(-1);
    }
    if syntax_checking::has_infinite_loops(&source_code) {
        println!("Error: Potential infinite loop detected!");
        process::exit(-1);
    }

    if args.with_dynamic_block {
        interpreter::run_with_dynamic_block(&source_code, &mut stdin, &mut stdout);
    } else {
        interpreter::run_with_fixed_block(&source_code, &mut stdin, &mut stdout, BLOCK_SIZE);
    }
}

fn file_exists(path: &str) -> bool {
    let metadata = fs::metadata(path);

    match metadata {
        Ok(m) => m.is_file(),
        Err(_e) => false,
    }
}
