use miette::{IntoDiagnostic, Result};
use std::{path::PathBuf, process::exit};
use structopt::StructOpt;
use ulang::{code_gen, parser::Parser};

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

fn main() -> Result<()> {
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
    let mut lexer = ulang::lexer::Lexer::from_path(opt.file.clone()).into_diagnostic()?;
    let tokens = lexer.tokenize()?;

    // if tokens.is_err() {
    //     use miette::miette;
    //     for error in tokens.unwrap_err() {
    //         eprintln!("{}",miette!(error));
    //     }
    //     exit(1);
    // }

    // let tokens = tokens.expect("Failed");
    println!("{:#?}", tokens);

    if opt.lex {
        exit(0);
    }

    let mut parser = Parser::new(tokens, lexer.path, lexer.content);
    let ast = parser.parse()?;
    println!("{:#?}", ast);

    if opt.parse {
        exit(0);
    }

    #[cfg(target_os = "linux")]
    let assembly = code_gen::generate_assembly(&ast, code_gen::TargetPlatform::X64Linux);
    #[cfg(not(target_os = "linux"))]
    let assembly = code_gen::generate_assembly(&ast, code_gen::TargetPlatform::MacOsX64);
    println!("{}", assembly.expect("Failed to generate ASM"));

    if opt.codegen {
        exit(0);
    }
    Ok(())
}
