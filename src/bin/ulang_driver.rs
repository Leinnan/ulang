use std::{fs, path::PathBuf, process::exit};
use structopt::StructOpt;
use ulang::parser::Parser;

/// Simple C lang compiler driver
#[derive(StructOpt, Debug)]
struct Opt {
    /// run the lexer, but stop before parsing
    #[structopt(long)]
    lex: bool,
    /// run the lexer and parser, but stop before assembly generation
    #[structopt(long)]
    parse: bool,
    /// perform lexing, parsing and assembly generation, but stop before code emission
    #[structopt(long)]
    codegen: bool,
    /// File to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

impl Opt {
    fn is_valid(&self) -> bool {
        let mut counter = 0;
        if self.lex {
            counter += 1;
        }
        if self.parse {
            counter += 1;
        }
        if self.codegen {
            counter += 1;
        }

        counter <= 1 && self.file.exists()
    }
}

fn main() {
    let opt = Opt::from_args();
    if !opt.is_valid() {
        if opt.file.exists() {
            eprintln!(
                "Selected multiple options, only one option can be selected at the time: {:?}",
                &opt
            );
        } else {
            eprintln!("File \"{}\" does not exists!", opt.file.display());
        }
        exit(1);
    }
    let tokens =
        ulang::lexer::tokenizer(fs::read_to_string(opt.file).expect("Could not read the file"));
    println!("{:#?}", tokens);

    if opt.lex {
        exit(0);
    }

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error during parsing: {}", e);
            exit(2);
        }
    };
    println!("{:#?}", ast);

    if opt.parse {
        exit(0);
    }
}
