use std::{env::args, fs, path::PathBuf};

use lexer::tokenizer;

pub mod lexer;

fn main() {
    let args: Vec<String> = args().collect();
    assert!(
        args.len() > 1 && args.len() < 4,
        "Program takes only path argument"
    );
    let i = if args.get(1).expect("msg").starts_with("--") {
        2
    } else {
        1
    };
    let program_path: PathBuf = args.get(i).expect("").into();

    let err = format!("Cannot read file, args: {:?}", &args);
    let tokens = tokenizer(fs::read_to_string(program_path).expect(&err));
    println!("{:#?}", tokens);
}
