use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::lexer::VarDest;
use crate::lexer::TokenCompData;
use thiserror::Error;
use std::fs::File;
use std::io::Write;

pub use crate::parser::parserTree::*;

#[derive(Debug, Error, Clone)]
pub enum CompilerError {
    #[error("(CompilerError) Unidentified error thrown. Reconsider life.")]
    UnidentifiedError,
    
    #[error("(CompilerError) No main procedure.")]
    NoMainProc,

    #[error("(CompilerError) Unable to open .kasm outfile '{0}'.")]
    UnableToOpenOutFile(String),

    #[error("(CompilerError) Unable to wrtie to .kasm outfile '{0}'.")]
    UnableToWriteOutFile(String),

    #[error("(CompilerError) Unimplemented ExpressionType of '{0:?}'..")]
    UnimplementedExprType(ExpressionType),

    #[error("(CompilerError) Unimplemented LineType of '{0:?}'.")]
    UnimplementedLineType(LineType),

    #[error("(CompilerError) Unimplemented VarDest of '{0:?}'.")]
    UnimplementedVarDest(VarDest),

    #[error("(CompilerError) Unimplemented DataAllocationType of '{0:?}'.")]
    UnimplementedDataAllocType(DataAllocationType),

    #[error("(CompilerError) Unimplemented TokenType of '{0:?}'.")]
    UnimplementedTokenType(TokenType),

    #[error("(CompilerError) Unimplemented argument type of '{0:?}'.")]
    UnimplementedArgumentType(ExpressionOutLocation),

    #[error("(CompilerError) Unimplemented register '{0}'.")]
    UnimplementedReg(String),

    #[error("(CompilerError) Unimplemented Embedded Procedure '{0}'.")]
    UnimplementedEmbeddedFunction(String),

    #[error("(CompilerError) Stand alone token of {1} of type {0:?}.")]
    InvalidStandAloneToken(TokenType, String),

    #[error("(CompilerError) Invalid assignment to token of {1} of type {0:?}.")]
    InvalidAssignment(TokenType, String),

    #[error("(CompilerError) Could not move {0:?} to {1:?} as there is no implemented opcode.")]
    InvalidMove(ExpressionOutLocation, ExpressionOutLocation),

    #[error("(CompilerError) Invalid procedure call of {0}.")]
    InvalidProcCall(String),

    #[error("(CompilerError) Invalid argument count of {0} calling {1}.")]
    InvalidArgCount(usize, String),

    #[error("(CompilerError) Invalid store of {0:?} into {1:?}.")]
    InvalidStore(ExpressionOutLocation, ExpressionOutLocation),

    #[error("(CompilerError) Invalid literal address {0}.")]
    InvalidAddress(String),

    #[error("(CompilerError) Invalid buffer indexing.")]
    InvalidBufferIndexing,
    
    #[error("(CompilerError) Register {0:?} overriden within expression with operator '{1}'.")]
    RegOverridden(ExpressionOutLocation, String),

    #[error("(CompilerError) Encountered blank expression.")]
    EncounteredBlankExpression,

    #[error("(CompilerError) The first argument of a syscall must ALWAYS be single byte literal, instead found {0:?}.")]
    SysArg1LiteralEnforce(ExpressionOutLocation),

    #[error("(CompilerError) Syscall provided with no arguments.")]
    SysArgEnforce(usize),

    #[error("(CompilerError) Exit provided with {0} arguments instead of 1.")]
    ExitArgEnforce(usize),
}

#[derive(Default, Debug, PartialEq, Clone)]
enum ExpressionOutLocation{
    #[default]
    None,
    RegisterA,
    RegisterX,
    RegisterY,
    Stack(u8),
    Heap(u16),
    Static(String),
    Literal(String),
    StringLiteral(String)
}

impl ExpressionOutLocation {
    pub fn reg(s: &str) -> Option<ExpressionOutLocation> {
        match s {
            "_A" => Some(ExpressionOutLocation::RegisterA),
            "_X" => Some(ExpressionOutLocation::RegisterX),
            "_Y" => Some(ExpressionOutLocation::RegisterY),
            _ => None,
        }
    }
}

fn moveOutTo(
    start_loc: ExpressionOutLocation,
    dest: ExpressionOutLocation,
) -> Result<String, CompilerError>{
    let mut expressionString: String = Default::default();
    match dest {
        ExpressionOutLocation::RegisterA => {
            match start_loc {
                ExpressionOutLocation::Literal(l) => {
                    expressionString.push_str("LDAC ");
                    expressionString.push_str(&l);
                    expressionString.push_str(";\n");
                }
                ExpressionOutLocation::RegisterX =>{
                    expressionString.push_str("TXA;\n");
                }
                ExpressionOutLocation::RegisterY =>{
                    expressionString.push_str("TYA;\n");
                }
                ExpressionOutLocation::RegisterA =>{
                    println!("THROW WARNING, MOVING _A INTO _A!");
                }
                ExpressionOutLocation::Stack(addr) =>{
                    expressionString.push_str("LDAS ");
                    expressionString.push_str(&addr.to_string());
                    expressionString.push_str(";\n");
                }
                ExpressionOutLocation::Heap(addr) =>{
                    expressionString.push_str("LDA ");
                    expressionString.push_str(&addr.to_string());
                    expressionString.push_str(";\n");
                }
                _ => return Err(CompilerError::InvalidMove(start_loc, dest)),
            }
        }
        ExpressionOutLocation::RegisterY => {
            match start_loc {
                ExpressionOutLocation::Literal(l) => {
                    expressionString.push_str("LDYC ");
                    expressionString.push_str(&l);
                    expressionString.push_str(";\n");
                }
                ExpressionOutLocation::RegisterX =>{
                    expressionString.push_str("TXY;\n");
                }
                ExpressionOutLocation::RegisterA =>{
                    expressionString.push_str("TAY;\n");
                }
                ExpressionOutLocation::Stack(addr) =>{
                    expressionString.push_str("LDYS ");
                    expressionString.push_str(&addr.to_string());
                    expressionString.push_str(";\n");
                }
                ExpressionOutLocation::Heap(addr) =>{
                    expressionString.push_str("LDY ");
                    expressionString.push_str(&addr.to_string());
                    expressionString.push_str(";\n");
                }
                _ => return Err(CompilerError::InvalidMove(start_loc, dest)),
            }
        }
        ExpressionOutLocation::Heap(addr) => {
            match start_loc {
                ExpressionOutLocation::Stack(s_addr) =>{
                    expressionString.push_str("STRC ");
                    expressionString.push_str(&addr.to_string());
                    expressionString.push_str(" ");
                    expressionString.push_str(&(0x0100+(s_addr as u16)).to_string());
                    expressionString.push_str(";\n");
                }
                ExpressionOutLocation::Heap(addr_origin) =>{
                    expressionString.push_str(
                        &moveOutTo(start_loc.clone(), ExpressionOutLocation::RegisterY)?
                    );

                    expressionString.push_str(
                        &moveOutTo(ExpressionOutLocation::RegisterY, dest.clone())?
                    );
                }
                ExpressionOutLocation::RegisterY => {
                    expressionString.push_str("STY ");
                    expressionString.push_str(&addr.to_string());
                    expressionString.push_str(";\n");
                }
                ExpressionOutLocation::RegisterA => {
                    expressionString.push_str("STA ");
                    expressionString.push_str(&addr.to_string());
                    expressionString.push_str(";\n");
                }
                ExpressionOutLocation::Literal(_) =>{
                    expressionString.push_str(
                        &moveOutTo(start_loc.clone(), ExpressionOutLocation::RegisterY)?
                    );

                    expressionString.push_str(
                        &moveOutTo(ExpressionOutLocation::RegisterY, dest.clone())?
                    );
                }
                ExpressionOutLocation::Static(label) => {
                    expressionString.push_str("STRC ");
                    expressionString.push_str(&addr.to_string());
                    expressionString.push(' ');
                    expressionString.push_str(&label);
                    expressionString.push_str(";\n");
                }
                _ => return Err(CompilerError::InvalidMove(start_loc, dest)),
            }
        }
        ExpressionOutLocation::Stack(s_addr) => {
            match start_loc {
                ExpressionOutLocation::Literal(_) =>{
                    expressionString.push_str(
                        &moveOutTo(start_loc.clone(), ExpressionOutLocation::RegisterY)?
                    );

                    expressionString.push_str(
                        &moveOutTo(ExpressionOutLocation::RegisterY, dest.clone())?
                    );
                }
                ExpressionOutLocation::RegisterY => {
                    expressionString.push_str("STYS ");
                    expressionString.push_str(&s_addr.to_string());
                    expressionString.push_str(";\n");
                }
                ExpressionOutLocation::RegisterA => {
                    expressionString.push_str("STAS ");
                    expressionString.push_str(&s_addr.to_string());
                    expressionString.push_str(";\n");
                }
                ExpressionOutLocation::Heap(addr) =>{
                    expressionString.push_str("STSH ");
                    expressionString.push_str(&s_addr.to_string());
                    expressionString.push_str(" ");
                    expressionString.push_str(&addr.to_string());
                    expressionString.push_str(";\n");
                }
                _ => return Err(CompilerError::InvalidMove(start_loc, dest)),
            }
        }
        ExpressionOutLocation::Static(ref label) => {
            match start_loc {
                ExpressionOutLocation::StringLiteral(ref s) => {
                    expressionString.push_str("LABEL ");
                    expressionString.push_str(label);
                    expressionString.push_str("\nRAW\n\"");
                    let mut lastWasEscape = false;
                    let mut i : usize = 1;
                    while i < s.len()-1 {
                        if s.as_bytes()[i] != b'\\'{
                            expressionString.push(s.as_bytes()[i] as char);
                        }
                        else {
                            i+=1;
                            match s.as_bytes()[i] {
                                b'n' => {
                                    expressionString.push_str("\" 10 ");
                                    if i != s.len()-2 {
                                        expressionString.push('"');
                                    }
                                    lastWasEscape = true;
                                }
                                _ =>  expressionString.push(s.as_bytes()[i] as char)
                            }
                        }
                        i+=1;
                    }
                    if !lastWasEscape {expressionString.push_str("\" ");}
                    expressionString.push_str("0\nEND\n");
                }
                _ => return Err(CompilerError::InvalidMove(start_loc, dest)),
            }
        }
        _ => return Err(CompilerError::InvalidMove(start_loc, dest))
    }
    return Ok(expressionString);
}

#[macro_export]
macro_rules! grabVariableCompNP {
    ($tuple: expr, $program: expr) => ({
        let mut variable = match $tuple.1 {
            VarDest::Heap => Ok($program.heap_variables[$tuple.0]),
            VarDest::ProgramStatic => Ok($program.static_variables[$tuple.0]),
            _ => Err(CompilerError::UnimplementedVarDest($tuple.1)),
        }; 
        variable
    })
}

fn evaluateExpr<'a>(
    expr: Expression<'a>,
    program: &'a Program, 
    current_proc: &'a Procedure
) -> Result<(String, ExpressionOutLocation), CompilerError> {
    #[macro_export]
    macro_rules! grabVariableComp {
        ($tuple: expr, $program: expr, $current_proc: expr) => ({
            let mut variable = match $tuple.1 {
                VarDest::CurrentProc => Ok($current_proc.variables[$tuple.0]),
                VarDest::Argument => Ok($current_proc.arguments[$tuple.0]),
                VarDest::Heap => Ok($program.heap_variables[$tuple.0]),
                VarDest::ProgramStatic => Ok($program.static_variables[$tuple.0]),
                _ => Err(CompilerError::UnimplementedVarDest($tuple.1)),
            }; 
            variable
        })
    }

    #[macro_export]
    macro_rules! procSyntax  {
        ($i: ident, $arg_count: ident, $exprTksLen: ident, $exprpkg: ident, $action: expr) => ({
            if expr.tks[$i].tk_data == "("{
                $i+=1;
                while expr.tks[$i].tk_data != ")" && $i < $exprTksLen{
                    // grab arg expression
                    let arg_start_index = $i;
                    while expr.tks[$i].tk_data != ")" && expr.tks[$i].tk_data != "," && $i < $exprTksLen {$i+=1;}
                    if $i < $exprTksLen{
                        $i-=1;
                        let $exprpkg = evaluateExpr(
                            Expression {
                                t:ExpressionType::Unspecified, 
                                tks: { 
                                    match arg_start_index-$i {
                                        0 => vec![expr.tks[$i]],
                                        _ => expr.tks[arg_start_index..$i].to_vec()
                                    }
                                }
                            },
                            program,
                            current_proc
                        )?;
                        
                        //action
                        $action;

                        $arg_count+=1;
                        $i+=2;

                        if expr.tks[$i-1].tk_data == ")" {break;}
                    } else {break;}
                }
            }
        })
    }
    
    let mut expressionString: String = Default::default();
    let mut expressionOutput: ExpressionOutLocation = ExpressionOutLocation::None;
    let mut startingIndex: usize = 0;
    let mut args: Vec<ExpressionOutLocation> = vec![];
    let exprTksLen = expr.tks.len();
    if exprTksLen == 1 {
        expressionOutput = match expr.tks[0].tk_type {
            TokenType::Variable => {
                let var = grabVariableComp!(expr.tks[0].tk_comp_data.var().ok_or(CompilerError::UnidentifiedError)?, program, current_proc)?;
                let o = match var.t.a{
                    DataAllocationType::Stack(addr) => ExpressionOutLocation::Stack(addr),
                    DataAllocationType::Heap(addr) => ExpressionOutLocation::Heap(addr),
                    DataAllocationType::Static => ExpressionOutLocation::Static(var.label.to_string()),
                    _ => return Err(CompilerError::UnimplementedDataAllocType(var.t.a)),
                };
                o
            }
            TokenType::CharLiteral |
            TokenType::NumberLiteral |
            TokenType::HexNumberLiteral => {
                ExpressionOutLocation::Literal(expr.tks[0].tk_data.to_string())
            }
            // using a procedure as a label
            TokenType::ProcedureCall => {
                ExpressionOutLocation::Static(expr.tks[0].tk_data.to_string())
            }
            TokenType::Register => {
                ExpressionOutLocation::reg(expr.tks[0].tk_data)
                    .ok_or(CompilerError::UnimplementedReg(expr.tks[0].tk_data.to_string()))?
            }
            _ => return Err(CompilerError::InvalidStandAloneToken(expr.tks[0].tk_type, expr.tks[0].tk_data.to_string())),
        };
    } else if exprTksLen == 0 {
        return Err(CompilerError::EncounteredBlankExpression);
    } else if expr.t == ExpressionType::Assignment {
        startingIndex = 2;

        match expr.tks[0].tk_type {
            TokenType::Variable => {
                // get variable
                let var = grabVariableComp!(expr.tks[0].tk_comp_data.var().ok_or(CompilerError::UnidentifiedError)?, program, current_proc)?;
                let o = match var.t.a{
                    //when stack
                    DataAllocationType::Stack(addr) => {
                        match var.t.v {
                            // if buffer, offset for index
                            DataValueType::Buffer => {
                                if expr.tks.len() > 3 && 
                                    expr.tks[1].tk_data == "[" &&
                                    expr.tks[3].tk_data == "]"
                                {
                                    startingIndex = 5;
                                    ExpressionOutLocation::Stack(
                                        addr + {
                                            match expr.tks[2].tk_data.parse::<u8>() {
                                                Ok(v) => v,
                                                Err(e) => return Err(CompilerError::InvalidAddress(expr.tks[2].tk_data.to_string())),
                                            }
                                        }
                                    )
                                }
                                else {return Err(CompilerError::InvalidBufferIndexing);}
                            }
                            //return stack addr
                            _ => ExpressionOutLocation::Stack(addr),
                        }
                    }
                    //when heap
                    DataAllocationType::Heap(addr) => ExpressionOutLocation::Heap(addr),
                    _ => return Err(CompilerError::UnimplementedDataAllocType(var.t.a)),
                };

                //eval expression
                let exprpkg = evaluateExpr(
                    Expression {t:ExpressionType::Unspecified, tks:expr.tks[startingIndex..exprTksLen].to_vec()},
                    program,
                    current_proc
                )?;
                expressionString.push_str(&exprpkg.0);
                expressionOutput = exprpkg.1;

                //print move
                expressionString.push_str(
                    &moveOutTo(expressionOutput.clone(), o)?
                );
            }
            TokenType::Register => {
                //eval expression
                let exprpkg = evaluateExpr(
                    Expression {t:ExpressionType::Unspecified, tks:expr.tks[startingIndex..exprTksLen].to_vec()},
                    program,
                    current_proc
                )?;
                expressionString.push_str(&exprpkg.0);
                expressionOutput = exprpkg.1;

                //print move
                expressionString.push_str(
                    &moveOutTo(
                        expressionOutput.clone(),
                        ExpressionOutLocation::reg(expr.tks[0].tk_data)
                            .ok_or(CompilerError::UnimplementedReg(expr.tks[0].tk_data.to_string()))?
                    )?
                );
            }
            _ => return Err(CompilerError::InvalidAssignment(expr.tks[0].tk_type, expr.tks[0].tk_data.to_string()))
        } 
        
    } else {
        let mut i: usize = 0;
        while i < exprTksLen {
            let tk = expr.tks[i];
            match tk.tk_type {
                TokenType::Variable => {
                    let var = grabVariableComp!(expr.tks[i].tk_comp_data.var().ok_or(CompilerError::UnidentifiedError)?, program, current_proc)?;
                    let o = match var.t.a{
                        DataAllocationType::Stack(addr) => {
                            match var.t.v {
                                // if buffer, offset for index
                                DataValueType::Buffer => {
                                    if expr.tks.len() > i+3 && 
                                        expr.tks[i+1].tk_data == "[" &&
                                        expr.tks[i+3].tk_data == "]"
                                    {   
                                        i+=3;
                                        ExpressionOutLocation::Stack(
                                            addr + {
                                                match expr.tks[i-1].tk_data.parse::<u8>() {
                                                    Ok(v) => v,
                                                    Err(e) => return Err(CompilerError::InvalidAddress(expr.tks[i-1].tk_data.to_string())),
                                                }
                                            }
                                        )
                                    }
                                    else {return Err(CompilerError::InvalidBufferIndexing);}
                                }
                                //return stack addr
                                _ => ExpressionOutLocation::Stack(addr),
                            }
                        }
                        DataAllocationType::Heap(addr) => ExpressionOutLocation::Heap(addr),
                        _ => return Err(CompilerError::UnimplementedDataAllocType(var.t.a)),
                    };
                    args.push(o);
                }
                TokenType::EmbeddedFunction =>{
                    match tk.tk_data {
                        "store" => {
                            // make function syntax parser into macro
                            // load up args
                            i = 1;
                            let mut args: Vec<ExpressionOutLocation> = vec![];
                            let mut arg_count = 0;
                            procSyntax!(i, arg_count, exprTksLen, exprpkg, {
                                args.push(exprpkg.1.clone());
                            });
                            //aftermath
                            if arg_count != 2 {
                                return Err(CompilerError::InvalidArgCount(arg_count, tk.tk_data.to_string()));
                            }

                            match &args[1] {
                                ExpressionOutLocation::Literal(l) => {
                                    expressionString.push_str(
                                        &moveOutTo(
                                            args[0].clone(), 
                                            ExpressionOutLocation::Heap({
                                                if !l.starts_with("0x") {
                                                    match l.parse::<u16>() {
                                                        Ok(v) => v,
                                                        Err(e) => return Err(CompilerError::InvalidAddress(l.clone())),
                                                    }
                                                }
                                                else{
                                                    match u16::from_str_radix(l.trim_start_matches("0x"), 16) {
                                                        Ok(v) => v,
                                                        Err(e) => return Err(CompilerError::InvalidAddress(l.clone())),
                                                    }
                                                }
                                            })
                                        )?
                                    );
                                }
                                _ => return Err(CompilerError::InvalidStore(args[0].clone(), args[1].clone()))
                            }
                        }
                        "sys" => {
                            // load up args
                            // no returns
                            i = 1;
                            let mut args: Vec<ExpressionOutLocation> = vec![];
                            let mut arg_count = 0;
                            procSyntax!(i, arg_count, exprTksLen, exprpkg, {
                                args.push(exprpkg.1.clone());
                                if arg_count > 0 {
                                    expressionString.push_str(&exprpkg.0);
                                    expressionString.push_str(
                                        &moveOutTo(exprpkg.1, ExpressionOutLocation::Heap(0xFFFE))?
                                    );
                                }
                            });
                            expressionString.push_str("SYS ");
                            if args.len() < 1 {
                                return Err(CompilerError::SysArgEnforce(args.len()));
                            }
                            match &args[0]{
                                ExpressionOutLocation::Literal(l) => expressionString.push_str(&l),
                                _ => return Err(CompilerError::SysArg1LiteralEnforce(args[0].clone())),
                            }
                            expressionString.push_str(";\n");
                        }
                        "exit" => {
                            // load up args
                            // no returns
                            i = 1;
                            let mut args: Vec<ExpressionOutLocation> = vec![];
                            let mut arg_count = 0;
                            procSyntax!(i, arg_count, exprTksLen, exprpkg, {
                                args.push(exprpkg.1.clone());
                            });

                            if args.len() < 1 || args.len() > 1 {
                                return Err(CompilerError::ExitArgEnforce(args.len()));
                            }

                            expressionString.push_str(
                                &moveOutTo(args[0].clone(), ExpressionOutLocation::RegisterA)?
                            );

                            expressionString.push_str("BRK;\n");
                        }
                        &_ => return Err(CompilerError::UnimplementedEmbeddedFunction(tk.tk_data.to_string()))
                    }
                }
                TokenType::ProcedureCall => {
                    // support returns later
                    let _called_proc = program.procs.as_slice().into_iter()
                        .find(|&p| p.label == tk.tk_data)
                        .ok_or(CompilerError::InvalidProcCall(tk.tk_data.to_string()));
                    
                    // load up args
                    let argSlots = [
                        ExpressionOutLocation::Heap(0x0005),
                        ExpressionOutLocation::Heap(0x0004),
                        ExpressionOutLocation::Heap(0x0003),
                        ExpressionOutLocation::Heap(0x0002),
                        ExpressionOutLocation::Heap(0x0001),
                        ExpressionOutLocation::Heap(0x0000),
                    ];

                    // proc call syntax
                    let mut arg_count = 0;
                    i+=1;
                    procSyntax!(i, arg_count, exprTksLen, exprpkg, {
                        expressionString.push_str(&exprpkg.0);
                        expressionString.push_str(
                            &moveOutTo(exprpkg.1, argSlots[arg_count].clone())?
                        );
                    });

                    // push jsr
                    expressionString.push_str("JSR ");
                    expressionString.push_str(tk.tk_data);
                    expressionString.push_str(";\n");
                }
                TokenType::OpAdd => {
                    // eval any exprs after (this reverses priority, fix later)
                    let exprpkg = evaluateExpr(
                        Expression {t:ExpressionType::Unspecified, tks:expr.tks[(i+1)..exprTksLen].to_vec()},
                        program,
                        current_proc
                    )?;

                    //check if both args exist
                    let mut arg2: ExpressionOutLocation = ExpressionOutLocation::None;
                    if args.len() < 1 {
                        return Err(CompilerError::UnidentifiedError);
                    }

                    // put arg1 in A reg
                    expressionString.push_str(&exprpkg.0);
                    if args[0] == ExpressionOutLocation::RegisterA && exprpkg.1 == ExpressionOutLocation::RegisterA {
                        return Err(CompilerError::RegOverridden(ExpressionOutLocation::RegisterA, "+".to_string()));
                    }
                    else if exprpkg.1 == ExpressionOutLocation::RegisterA {
                        arg2 = args[0].clone();
                    }
                    else if args[0] == ExpressionOutLocation::RegisterA{
                        arg2 = exprpkg.1;
                    }
                    else {
                        expressionString.push_str(
                            &moveOutTo(args[0].clone(), ExpressionOutLocation::RegisterA)?
                        );
                        arg2 = exprpkg.1;
                    }

                    //do a diff add based on arg2 type
                    match arg2 {
                        ExpressionOutLocation::Stack(addr) => {
                            expressionString.push_str(
                                &moveOutTo(arg2.clone(), ExpressionOutLocation::Heap(0x0000))?
                            );
                            expressionString.push_str("ADC 0x00;\n");
                        }
                        ExpressionOutLocation::Literal(l) => {
                            expressionString.push_str("ADCC ");
                            expressionString.push_str(&l);
                            expressionString.push_str(";\n");
                        }
                        _ => return Err(CompilerError::UnimplementedArgumentType(arg2))
                    }

                    expressionOutput = ExpressionOutLocation::RegisterA;

                    break;
                }
                _ => return Err(CompilerError::UnimplementedTokenType(tk.tk_type))
            }
            i+=1;
        }
    }

    match expr.t{
        // store output in A reg on return
        ExpressionType::Return => {
            expressionString.push_str(
                &moveOutTo(expressionOutput.clone(), ExpressionOutLocation::RegisterA)?
            );
        }
        _ => {}
    }
    if expressionOutput == ExpressionOutLocation::None && args.len() == 1 {
        expressionOutput = args[0].clone();
    }
    return Ok((expressionString, expressionOutput));
}

pub fn runCompiler<'a>(program: Program<'a>, out_file: &'a str) -> Result<(), CompilerError>{
    let mut procs: Vec<String> = vec![]; 
    let mut header: String = Default::default();
    let mut label_header: String = Default::default();

    let mut hasMain = false;
    println!("{program:#2?}");

    if program.expressions.len() != 0 {
        header.push_str("__START_HEADER__\n");
    }

    for expr in &program.expressions {
        match expr.t {
            ExpressionType::Assignment => {
                let expressionOutput = match expr.tks[2].tk_type {
                    TokenType::CharLiteral |
                    TokenType::NumberLiteral |
                    TokenType::HexNumberLiteral => {
                        ExpressionOutLocation::Literal(expr.tks[2].tk_data.to_string())
                    }
                    TokenType::StringLiteral => {
                        ExpressionOutLocation::StringLiteral(expr.tks[2].tk_data.to_string())
                    }
                    TokenType::Symbol => {
                        if expr.tks[2].tk_data == "[" {
                            //array literal
                            let var = grabVariableCompNP!(
                                expr.tks[0].tk_comp_data.var().ok_or(
                                    CompilerError::UnidentifiedError
                                )?, 
                                program
                            )?;
                            
                            if var.t.a == DataAllocationType::Static && var.t.v == DataValueType::Buffer {
                                label_header.push_str("LABEL ");
                                label_header.push_str(expr.tks[0].tk_data);
                                label_header.push_str("\nRAW\n");
                                let mut i = 3;
                                while i < expr.tks.len() && expr.tks[i].tk_data != "]" {
                                    label_header.push_str(expr.tks[i].tk_data);
                                    label_header.push(' ');
                                    i+=1;
                                }
                                label_header.push_str("\nEND\n");
                            } else {
                                return Err(CompilerError::InvalidStandAloneToken(expr.tks[2].tk_type, expr.tks[2].tk_data.to_string()));
                            }
                            continue;
                        } else {
                            return Err(CompilerError::InvalidStandAloneToken(expr.tks[2].tk_type, expr.tks[2].tk_data.to_string()));
                        }
                    }
                    _ => return Err(CompilerError::InvalidStandAloneToken(expr.tks[2].tk_type, expr.tks[2].tk_data.to_string())),
                };
                let var = grabVariableCompNP!(
                    expr.tks[0].tk_comp_data.var().ok_or(
                        CompilerError::UnidentifiedError
                    )?, 
                    program
                )?;
                let o = match var.t.a{
                    DataAllocationType::Heap(addr) => ExpressionOutLocation::Heap(addr),
                    DataAllocationType::Static => ExpressionOutLocation::Static(var.label.to_string()),
                    _ => return Err(CompilerError::UnimplementedDataAllocType(var.t.a)),
                };
                match o {
                    ExpressionOutLocation::Static(_) => label_header.push_str(
                        &moveOutTo(expressionOutput.clone(), o)?
                    ),
                    _ => header.push_str(
                        &moveOutTo(expressionOutput.clone(), o)?
                    ),
                }
            }
            _ => return Err(CompilerError::UnimplementedExprType(expr.t))
        }
    }

    if program.expressions.len() != 0 {
        header.push_str("__END_HEADER__\n");
    }

    for p in &program.procs {
        // println!("{p:#2?}");
        let mut contents: String = Default::default();

        if p.label == "main" {
            contents.push_str("LABEL __MAIN__\n");
            hasMain = true;
        }
        else {
            contents.push_str("LABEL ");
            contents.push_str(p.label);
            contents.push('\n');
        }

        if p.allocated_bytes != 0 {
            contents.push_str("SAL ");
            contents.push_str(&p.allocated_bytes.to_string());
            contents.push_str(";\n");
        }
        
        for index in 0..p.lines.len() as usize{
            match p.lines[index].t {
                LineType::Expression => {
                    match p.expressions[p.lines[index].index].t {
                        ExpressionType::Return => {
                            if p.expressions[p.lines[index].index].tks.len() != 0 {
                                contents.push_str(
                                    evaluateExpr(
                                        p.expressions[p.lines[index].index].clone(),
                                        &program,
                                        p
                                    )?.0.as_str()
                                );
                            }
                            if p.allocated_bytes != 0 {
                                contents.push_str("DAL ");
                                contents.push_str(&p.allocated_bytes.to_string());
                                contents.push_str(";\n");
                            }
                            if p.label == "main" {
                                contents.push_str("BRK;\n");
                            }
                            else {
                                contents.push_str("RTS;\n");
                            }
                        }
                        ExpressionType::Unspecified |
                        ExpressionType::Assignment => {
                            contents.push_str(
                                evaluateExpr(
                                    p.expressions[p.lines[index].index].clone(),
                                    &program,
                                    p
                                )?.0.as_str()
                            );
                        }
                        _ => return Err(CompilerError::UnimplementedExprType(p.expressions[p.lines[index].index].t))
                    }
                }
                _ => return Err(CompilerError::UnimplementedLineType(p.lines[index].t))
            }
        }

        if p.label == "main" {
            procs.insert(0, contents.clone());
        }
        else {
            procs.push(contents.clone());
        }
        // println!("{contents}");
    }

    if !hasMain {
        return Err(CompilerError::NoMainProc);
    }

    let mut out_file_path: File = match File::create(out_file) {
        Ok(f) => f,
        Err(e) => return Err(CompilerError::UnableToOpenOutFile(out_file.to_string()))
    };

    match write!(out_file_path,"{}",header) { 
        Ok(_) => (),
        Err(e) => return Err(CompilerError::UnableToWriteOutFile(out_file.to_string()))
    };

    match write!(out_file_path,"{}",label_header) { 
        Ok(_) => (),
        Err(e) => return Err(CompilerError::UnableToWriteOutFile(out_file.to_string()))
    };

    for i in 1..procs.len() {
        match write!( out_file_path, "{}", procs[i]) {
            Ok(_) => (),
            Err(e) => return Err(CompilerError::UnableToWriteOutFile(out_file.to_string()))
        };
    }
    match write!( out_file_path, "{}", procs[0]) {
        Ok(_) => (),
        Err(e) => return Err(CompilerError::UnableToWriteOutFile(out_file.to_string()))
    };
    
    return Ok(());
}