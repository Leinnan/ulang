use clap::Parser;
use miette::{IntoDiagnostic, Result};
use std::{path::PathBuf, process::exit};
use ulang::assembly;

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
    /// perform lexing, parsing and tacky generation, but stop before code assembly
    #[arg(long)]
    tacky: bool,
    /// File to process
    file: PathBuf,
    /// Save to file
    save_path: Option<PathBuf>,
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
        if self.tacky {
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

    let mut tacky = ulang::tacky::Tacky::from_program_node(&ast).unwrap();
    let result = tacky.parse().unwrap();
    println!("\nTacky\n{:#?}", result);
    if opt.tacky {
        exit(0);
    }

    let asm_ast: ulang::assembly::AsmProgram = (&result).into();
    println!("ASM AST: {:#?}", asm_ast);

    let asm_replaced: ulang::assembly::AsmProgramWithReplacedPseudoRegisters = asm_ast.into();
    println!("ASM Replaced: {:#?}", asm_replaced);

    let asm_fixed: ulang::assembly::AsmProgramWithFixedInstructions = asm_replaced.into();
    println!("ASM Fixed: {:#?}", asm_fixed);

    #[cfg(target_os = "linux")]
    let target = assembly::TargetPlatform::X64Linux;
    #[cfg(not(target_os = "linux"))]
    let target = assembly::TargetPlatform::MacOsX64;

    let asm_final = asm_fixed.generate(target);
    println!("{}", asm_final.0);

    if opt.codegen {
        exit(0);
    }
    let path = opt.save_path.unwrap_or(opt.file.with_extension("s"));
    std::fs::write(&path, asm_final.0).expect("Failed to save file");
    use std::process::Command;
    let cmd = format!(
        "gcc {} -o {}",
        &path.display(),
        path.with_extension("").display()
    );
    println!("Running: {}", &cmd);
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &cmd])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;
    println!("result: {:?}", hello);
    Ok(())
}
