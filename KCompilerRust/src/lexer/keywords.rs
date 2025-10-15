pub static KEYWORDS: &[&str] = &[
    "static", "string", "const", "short", "buffer", "heap", 
    "LABEL", "raw", "end", "proc", "ret",
    "while", "if", "else", "void", "uint"
];

pub static EMBEDDED_FNS: &[&str] = &[
    "store", "sys", "exit"
];

pub static SYMBOLS: &[&str] = &[
    ";", ":", "(", ")", ",", "[", "]"
];

use crate::lexer::TokenType;

pub fn sliceToKeyword(s: &str) -> TokenType {
    match s{
        "static" => return TokenType::KeywordStatic,
        "string" => return TokenType::KeywordString,
        "buffer" => return TokenType::KeywordBuffer,
        "const" => return TokenType::KeywordConst,
        "short" => return TokenType::KeywordShort,
        "heap" => return TokenType::KeywordHeap,
        "LABEL" => return TokenType::KeywordLABEL,
        "raw" => return TokenType::KeywordRaw,
        "end" => return TokenType::KeywordEnd,
        "proc" => return TokenType::KeywordProc,
        "ret" => return TokenType::KeywordRet,
        "while" => return TokenType::KeywordWhile,
        "if" => return TokenType::KeywordIf,
        "else" => return TokenType::KeywordElse,
        "void" => return TokenType::KeywordVoid,
        "uint" => return TokenType::KeywordUint,
        &_ => todo!(),
    }
}

pub fn isKeyword(s: &str) -> bool{
    for w in KEYWORDS{
        if *w == s {return true;}
    }
    return false;
}

pub fn sliceToSymbol(s: &str) -> TokenType {
    match s{
        ";" => return TokenType::SymbolSemicolon,
        &_ => return TokenType::Symbol,
    }
}

pub fn charToSymbol(c: char) -> TokenType {
    match c{
        ';' => return TokenType::SymbolSemicolon,
        _ => return TokenType::Symbol,
    }
}

pub fn charToOp(c: char) -> TokenType {
    match c{
        '+' => return TokenType::OpAdd,
        '-' => return TokenType::OpSubtract,
        '=' => return TokenType::OpAssign,
        '!' => return TokenType::OpNot,
        _ => return TokenType::Op,
    }
}

pub fn strToOp(s: &str) -> TokenType {
    match s{
        "==" => return TokenType::OpEq,
        ">=" => return TokenType::OpGreatEq,
        "<=" => return TokenType::OpLessEq,
        "!=" => return TokenType::OpNEq,
        &_ => return TokenType::Op,
    }
}

pub fn isSymbol(s: &str) -> bool{
    for sym in SYMBOLS{
        if *sym == s {return true;}
    }
    return false;
}

pub fn isEmbeddedFn(s: &str) -> bool {
    for w in EMBEDDED_FNS{
        if *w == s {return true;}
    }
    return false;
}