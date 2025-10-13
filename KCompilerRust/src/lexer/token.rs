//Token type enum
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum TokenType{
    #[default]
    None,
    Register,
    UnidentifiedLabel,
    Variable,
    ProcedureCall,
    CharLiteral,
    StringLiteral,
    NumberLiteral,
    HexNumberLiteral,
    Symbol,
    SymbolSemicolon,
    Op,
    OpAssign,
    OpAdd,
    OpSubtract,
    KeywordString,
    KeywordUint,
    KeywordShort,
    KeywordBuffer,
    KeywordStatic,
    KeywordHeap,
    KeywordConst,
    KeywordLABEL,
    KeywordRaw,
    KeywordEnd,
    KeywordProc,
    KeywordRet,
    KeywordWhile,
    KeywordIf,
    KeywordElse,
    KeywordVoid,
    EmbeddedFunction
}

/*impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    }
}*/

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum VarDest {
    #[default]
    None,
    Heap,
    ProgramConst,
    ProgramStatic,
    Program,
    CurrentProc,
    Argument,
    Block(u8) //block is measured by distance from current proc scope, 0 is error
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum TokenCompData{
    #[default]
    None,
    Var(usize,VarDest)
}

impl TokenCompData {
    pub fn var(self) -> Option<(usize,VarDest)> {
        match self {
            TokenCompData::Var(i, d) => Some((i, d)),
            _ => None,
        }
    }
}

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct Token<'a>{
    pub tk_data: &'a str,
    pub tk_type: TokenType,
    pub tk_comp_data: TokenCompData,
}