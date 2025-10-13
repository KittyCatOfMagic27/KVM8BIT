// :)
#![allow(
    non_snake_case,
    unused_imports,
    dead_code
)]

pub mod lexer;
pub mod parser;
pub mod compiler;

use crate::lexer::LexerError;
use std::fs::*;
use std::io::Read;

fn loadFile<'a>(file_name: &'a str)->Result<String, LexerError>{
    let mut file_bundle = File::open(file_name)?;
    let mut file_data = String::new();
    file_bundle.read_to_string(&mut file_data)?;
    return Ok(file_data);
}

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = "../main.k";
    let out_file_name = "../program.kasm";
    // let file_name = "../ExamplesK/syntax.k";
    let file_contents: String = match loadFile(file_name) {
        Ok(file_data) => file_data,
        Err(e) => {
            println!("{e}");
            return Err(Box::new(e));
        }
    };
    let mut token_storage : Vec<lexer::Token<'_>> = Default::default();
    let _lexerOutput = match lexer::runLexer(&file_contents, & mut token_storage) {
        Ok(output) => output,
        Err(e) => {println!("Lexer failed: {e}"); return Err(Box::new(e));}
    };
    
    let mut parserWarnings : Vec<parser::ParserWarning> = vec![];
    let mut program : crate::parser::Program<'_> = Default::default();
    let _parserOutput = match parser::runParser(&mut token_storage, program, & mut parserWarnings) {
        Ok(output) => program = output,
        Err(e) => {println!("Parser failed: {e}"); return Err(Box::new(e));}
    };
    for w in parserWarnings {
        println!("Parser Threw Warning: {w}");
    }
    
    match compiler::runCompiler(program, &out_file_name) {
        Ok(output) => output,
        Err(e) => {println!("Compiler failed: {e}"); return Err(Box::new(e));}
    }

    return Ok(());
}