use std::{env::args, path::PathBuf, process::ExitStatus};

pub struct CompilerDriver {
    pub program_path: PathBuf,
}

impl CompilerDriver {
    fn run_preprocess(&self) {
        std::process::Command::new("gcc")
            .arg("-E")
            .arg("-P")
            .arg(&self.program_path)
            .output()
            .expect("Failed to execute command");
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    assert!(
        args.len() > 1 && args.len() < 4,
        "Program takes only path argument"
    );
    let program_path: PathBuf = args.get(1).expect("").into();
    assert!(program_path.exists(), "Program path must exists!");
    let compiler_driver = CompilerDriver { program_path };
    println!("Hello, world!");
}
