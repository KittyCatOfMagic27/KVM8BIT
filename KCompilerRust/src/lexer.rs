use std::fs::*;
use std::io::{self, Read};

//Custom Error Implementation
use thiserror::Error;

//includes
pub mod keywords;
pub mod token;

use crate::parser;

pub use crate::lexer::token::*;

// look into if you can implement line and character numbers into the errors
#[derive(Debug, Error)]
pub enum LexerError {
    #[error("(LexerError) Unidentified error thrown. Reconsider life.")]
    UnidentifiedError,

    #[error("(LexerError) Incorrect hex value.")]
    InvalidHexValue,
    
    #[error("(LexerError) Length of char literal is invalid.")]
    CharLengthInvalid,

    #[error("(LexerError) Size of a value is above 8 bits when it should only be 8 bits.")]
    InvalidValueSize8b,

    #[error("(LexerError) Size of a value is above 16 bits when it should only be 16 bits.")]
    InvalidValueSize16b,
    
    #[error("(LexerError) Attempting to use a register that does not exist.")]
    InvalidRegister,

    #[error("(LexerError) String literal not terminated with a second \".")]
    NonTerminatedString,

    #[error("(LexerError) Comment not terminated with a second #.")]
    NonTerminatedComment,

    #[error("(LexerError) File Not Read Error: {0}")]
    FileNotRead(#[from] io::Error)
}

fn tokenize<'a>(
    file_data: &'a String,
    start: usize,
    index: &usize,
    tokenType: TokenType
) -> Token<'a> {
    return Token { tk_type: tokenType, tk_data: &file_data[start..*index], tk_comp_data: TokenCompData::None};
}

fn getNextToken<'a,'b>(file_data: &'a String, index: &'b mut usize)->Result<Token<'a>, LexerError>{
    while file_data.as_bytes()[*index].is_ascii_whitespace(){*index+=1;}
    
    let mut outTk: Token<'a> = Default::default();
    let start: usize = *index;
    let mut c : char = file_data.as_bytes()[*index] as char;

    match c {
        // if starting with symbol
        '+' | '-' | '=' | '>' | '<' =>{
            *index+=1;

            return Ok(tokenize(
                file_data,
                start,
                index,
                keywords::charToOp(c)
            ));
        }
        '(' | ')' | ';' | ',' | ']' | '[' =>{
            *index+=1;

            return Ok(tokenize(
                file_data,
                start,
                index,
                keywords::charToSymbol(c)
            ));
        }
        '\'' =>{
            if (file_data.len() > *index+2) && file_data.as_bytes()[*index+2] == b'\''{ *index+=3; }
            else { return Err(LexerError::CharLengthInvalid); }

            return Ok(tokenize(
                file_data,
                start,
                index,
                TokenType::CharLiteral
            ));
        }
        '"' =>{
            *index+=1;

            while (file_data.len() > *index) && (file_data.as_bytes()[*index]!=b'"') {*index+=1;}
            
            if file_data.len() <= *index {return Err(LexerError::NonTerminatedString);}
            
            *index+=1;

            return Ok(tokenize(
                file_data,
                start,
                index,
                TokenType::StringLiteral
            ));
        }
        '#' =>{
            *index+=1;
            while (file_data.len() > *index) && (file_data.as_bytes()[*index]!=b'#') {*index+=1;}
            if file_data.len() <= *index {return Err(LexerError::NonTerminatedString);}
            *index+=1;
            return getNextToken(&file_data, index);
        }
        ':' =>{
            if (file_data.len() > *index+1) && file_data.as_bytes()[*index+1] == b':'{ *index+=1; }
            *index+=1;

            return Ok(tokenize(
                file_data,
                start,
                index,
                TokenType::Symbol
            ));
        }
        '_' =>{
            *index+=1;
            
            c = file_data.as_bytes()[*index] as char;
            if (file_data.len() <= *index+1) || (c != 'A' && c != 'X' && c != 'Y' && c != 'S') {
                return Err(LexerError::InvalidRegister);
            }
            
            *index+=1;

            return Ok(tokenize(
                file_data,
                start,
                index,
                TokenType::Register
            ));
        }
        _ => {
            while (file_data.len() > *index) && (!file_data.as_bytes()[*index].is_ascii_whitespace()) {
                c = file_data.as_bytes()[*index] as char;
                match c {
                    '+' | '-' | '=' | '>' | '<' | '(' | ')' | ';' | ',' | '\'' | '"' | '#' | ':' | '_' | ']' | '[' =>{ break; }
                    _ =>{*index+=1;}
                }
            }
        }
    }
    outTk.tk_data = &file_data[start..*index];

    use crate::lexer::keywords::sliceToKeyword;
    
    outTk.tk_type = match outTk.tk_data {
        kw if keywords::isKeyword(kw) => sliceToKeyword(kw),
        kw if keywords::isEmbeddedFn(kw) => TokenType::EmbeddedFunction,
        kw if kw.parse::<u16>().is_ok() => TokenType::NumberLiteral,
        kw if kw.parse::<f64>().is_ok() => return Err(LexerError::InvalidValueSize16b),
        kw if kw.starts_with("0x") =>{
            let s = &kw[2..];
            let number = match u64::from_str_radix(&s, 16) {
                Ok(n) => n,
                Err(_e) => return Err(LexerError::InvalidHexValue),
            };
            if number > u16::MAX as u64 {
                return Err(LexerError::InvalidValueSize16b);
            } 
            TokenType::HexNumberLiteral
        }
        _ => TokenType::UnidentifiedLabel
        
    };

    return Ok(outTk);
}

pub fn runLexer<'a>(file_contents: &'a String, token_storage: &mut Vec<Token<'a>>)->Result<(), LexerError>{
    let mut index: usize = 0;
    while file_contents.len() > index {
        let tk: Token = getNextToken(&file_contents, &mut index)?;
        token_storage.push(tk);
    }
    return Ok(());
}