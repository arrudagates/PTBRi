use std::{env::args, fs};

use anyhow::Result;

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod interpreter;
pub use interpreter::*;
mod types;
pub use types::*;
mod ast;
pub use ast::*;
mod error;
pub use error::*;
mod parser;
pub use parser::*;

pub fn main() -> Result<()> {
    let mut program = String::new();

    let args: Vec<String> = args().collect();

    if args.len() > 0 {
        if std::path::Path::new(&args[1]).is_file() {
            program = String::from_utf8_lossy(
                &fs::read(std::path::Path::new(&args[1])).expect("Failed to read file"),
            )
            .to_string();
        }
    }

    run(program)
}
