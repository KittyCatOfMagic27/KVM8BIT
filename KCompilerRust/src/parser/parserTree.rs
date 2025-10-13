use crate::lexer::Token;
use crate::parser::ParserError;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum DataValueType{
    #[default]
    Void,
    Uint,
    Short,
    Char,
    String,
    Buffer
}

impl DataValueType {
    pub fn size(self) -> Option<u8> {
        match self {
            DataValueType::Uint |
            DataValueType::Char => Some(1),
            DataValueType::Short => Some(2),
            DataValueType::String |
            DataValueType::Buffer => Some(2), //size of the pointer
            _ => None,
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum DataAllocationType{
    #[default]
    None,
    Heap(u16),
    Stack(u8),
    Static,
    Const
}

impl DataAllocationType {
    pub fn stack(self) -> Option<u8> {
        match self {
            DataAllocationType::Stack(addr) => Some(addr),
            _ => None,
        }
    }
    pub fn heap(self) -> Option<u16> {
        match self {
            DataAllocationType::Heap(addr) => Some(addr),
            _ => None,
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct DataType{
    pub a: DataAllocationType,
    pub v: DataValueType
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Variable<'a>{
    pub t: DataType,
    pub value: Option<&'a Token<'a>>,
    pub label: &'a str
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum ExpressionType{
    #[default]
    Unspecified,
    Return,
    Assignment
}

#[derive(Default, Debug)]
pub struct Expression<'a> {
    pub t: ExpressionType,
    pub tks: Vec<&'a Token<'a>>
}

impl <'a> Clone for Expression<'a> {
    fn clone(&self) -> Expression<'a> {
        let mut e : Expression<'a> = Default::default();
        e.t = self.t.clone();
        for tk in &self.tks {
            e.tks.push(tk);
        }
        return e;
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum LineType{
    #[default]
    Expression,
    Block
}

#[derive(Default, Debug)]
pub struct Line {
    pub index: usize,
    pub t: LineType
}

#[derive(Default, Debug)]
pub struct Procedure<'a>{
    pub allocated_bytes: u8,
    pub label: &'a str,
    pub retType: DataType,
    pub arguments: Vec<Variable<'a>>,
    pub variables: Vec<Variable<'a>>,
    pub expressions: Vec<Expression<'a>>,
    pub lines: Vec<Line>
}

#[derive(Default, Debug)]
pub struct Program<'a>{
    pub allocated_bytes: u16, //for heap vars
    pub heap_variables: Vec<Variable<'a>>,
    pub const_variables: Vec<Variable<'a>>,
    pub static_variables: Vec<Variable<'a>>,
    pub expressions: Vec<Expression<'a>>,
    pub procs: Vec<Procedure<'a>>
}

pub fn toValueType(s: &str) -> Result<DataValueType, ParserError>{
    match s {
        "void" => return Ok(DataValueType::Void),
        "uint" => return Ok(DataValueType::Uint),
        "short" => return Ok(DataValueType::Short),
        "char" => return Ok(DataValueType::Char),
        "string" => return Ok(DataValueType::String),
        &_ => return Err(ParserError::UnidentifiedType(s.to_string()))
    }
}