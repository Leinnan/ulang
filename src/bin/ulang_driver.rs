use clap::Parser;
use miette::{IntoDiagnostic, Result};
use std::{path::PathBuf, process::exit};
use ulang::code_gen;

/// Simple C lang compiler driver
#[derive(Parser, Debug)]
#[command(version, about, long_about = "Test compiler")]
#[command(propagate_version = true)]
struct UlangDriver {
    /// run the lexer, but stop before parsing
    #[arg(long)]
    lex: bool,
    /// run the lexer and parser, but stop before assembly generation
    #[arg(long)]
    parse: bool,
    /// perform lexing, parsing and assembly generation, but stop before code emission
    #[arg(long)]
    codegen: bool,
    /// File to process
    file: PathBuf,
}

impl UlangDriver {
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
    let opt = UlangDriver::parse();
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

    let mut parser = ulang::parser::Parser::new(tokens, lexer.path, lexer.content);
    let ast = parser.parse()?;
    println!("{:#?}", ast);

    if opt.parse {
        exit(0);
    }

    #[cfg(target_os = "linux")]
    let assembly = code_gen::generate_assembly(&ast, code_gen::TargetPlatform::X64Linux);
    #[cfg(not(target_os = "linux"))]
    let assembly = code_gen::generate_assembly(&ast, code_gen::TargetPlatform::MacOsX64);
    match &assembly {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }

    if opt.codegen {
        exit(0);
    }
    Ok(())
}
