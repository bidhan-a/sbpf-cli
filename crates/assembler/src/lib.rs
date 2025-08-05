extern crate num_traits;
extern crate num_derive;
extern crate anyhow;

use std::path::Path;
use anyhow::{Error, Result};
use codespan_reporting::files::SimpleFile;

// Tokenizer and parser
pub mod parser;
pub mod lexer;
pub mod opcode;

// Error handling and diagnostics
pub mod macros;
pub mod errors;

// Intermediate Representation
pub mod astnode;
pub mod dynsym;

// ELF header, program, section
pub mod header;
pub mod program;
pub mod section;

// Debug info
pub mod debuginfo;

#[cfg(test)]
mod tests;

// Type aliases for error handling
pub type ParserError = String;
pub type ProgramError = String;
pub type TokenizerError = String;

pub use self::{
    parser::Parser,
    program::Program,
    lexer::tokenize,
};

pub fn assemble(src: &str, deploy: &str) -> Result<()> {
    let source_code = std::fs::read_to_string(src)?;
    let file = SimpleFile::new(src.to_string(), source_code.clone());

    let tokens = match tokenize(&source_code) {
        Ok(tokens) => tokens,
        Err(e) => return Err(Error::msg(format!("Tokenizer error: {}", e))),
    };

    let mut parser = Parser::new(tokens, &file);
    let parse_result = match parser.parse() {
        Ok(program) => program,
        Err(e) => return Err(Error::msg(format!("Parser error: {}", e))),
    };

    let program = Program::from_parse_result(parse_result);

    let bytecode = program.emit_bytecode();

    let output_path = Path::new(deploy)
        .join(Path::new(src)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".s", ".so"));

    std::fs::write(output_path, bytecode)?;
    Ok(())
}
