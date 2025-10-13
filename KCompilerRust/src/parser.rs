use crate::lexer::Token;
use crate::lexer::TokenType;
use thiserror::Error;

pub mod parserTree;
pub use crate::parser::parserTree::*;

use crate::lexer::LexerError;
use crate::lexer::VarDest;
use crate::lexer::TokenCompData;

#[derive(Debug, Error, Clone)]
pub enum ParserError {
    #[error("(ParserError) Unidentified error thrown. Reconsider life.")]
    UnidentifiedError,

    #[error("(ParserError) Unidentified token '{0}'.")]
    UnidentifiedToken(String),
    
    #[error("(ParserError) Type '{0}' does not exist.")]
    UnidentifiedType(String),

    #[error("(ParserError) Missing para in procedure definition.")]
    NoParaProcDef,

    #[error("(ParserError) Para not terminated.")]
    NoParaNotTerminated,

    #[error("(ParserError) Ran into an excessive end statement.")]
    ExcessiveEndStatement,

    #[error("(ParserError) Missing end statement.")]
    MissingEndStatement,

    #[error("(ParserError) Missing terminating square bracket.")]
    MissingClosingSqBracket,

    #[error("(ParserError) Missing size in buffer declaration, this is done by '[SIZE]'.")]
    MissingSizeForBuffer,

    #[error("(ParserError) Size for buffer declaration is not a number, note hexidecimal is does not count.")]
    MissingSizeForBufferNotNumber,

    #[error("(ParserError) Symbol, keyword, or value incorrectly placed in expression.")]
    SymbolIncorrectlyInExpression,

    #[error("(ParserError) Unnecessary semicolon.")]
    UnnecessarySemicolon,

    #[error("(ParserError) Attempted to put an expression outside of a procedure.")]
    AttemptedExpressionInProgram,

    #[error("(ParserError) Attempted to put the variable '{0}' outside of a procedure.")]
    AttemptedVariableInProgram(String),

    #[error("(ParserError) Label '{0}' could not be identified.")]
    UnidentifiedLabel(String),

    #[error("(ParserError) The type '{0:?}' was placed in the middle of an expression.")]
    TypeInExpression(DataValueType),

    #[error("(ParserError) Value '{0}' found outside of expression.")]
    StrayValue(String),

    #[error("(ParserError) Assignment to nothing.")]
    StrayAssignment,

    #[error("(ParserError) Operator '{0}' not within expression or missing oprehands.")]
    StrayOperator(String),

    #[error("(ParserError) Variable '{0}' is redefined.")]
    RedefinitionOfVariable(String),

    #[error("(ParserError) Const variable '{0}' lacks an initial value (required).")]
    ConstNoInitial(String),

    #[error("(ParserError) Unimplemented DataAllocationType {0:?}, couldn't allocate the space.")]
    UnimplementedDataAllocType(DataAllocationType),

    #[error("(ParserError) Unimplemented VarDest {0:?}, couldn't query variable.")]
    UnimplementedVarDestType(VarDest),

    #[error("(ParserError) Unimplemented DataValueType {0:?}, couldn't find size.")]
    UnimplementedDataValueType(DataValueType),

    #[error("{0}")]
    LexError(String)
}

#[derive(Debug, Error)]
pub enum ParserWarning {
    #[error("(ParserWarning) Unidentified warning.")]
    WarningUnidentified,

    #[error("(ParserWarning) Variable '{0}' is defined without initial value being assigned.")]
    WarningNoInitialValue(String),
}

pub fn runParser<'a>(token_storage: &'a mut [Token<'a>], mut program: Program<'a>, warnings: & mut Vec<ParserWarning>)->Result<Program<'a>, ParserError>{
    
    #[macro_export]
    macro_rules! grabVariable {
        ($label: expr, $program: expr, $current_proc: expr) => ({
            let mut variable = match $current_proc{
                Some(p) => {
                    let mut fv = Default::default();
                    for v in &$program.procs[p].arguments{
                        if v.label == $label {fv = Some(v); break;}
                    }
                    if fv == Default::default() {
                        for v in &$program.procs[p].variables {
                            if v.label == $label {fv = Some(v); break;}
                        }
                    }
                    fv
                }
                None => None
            };

            if variable.is_none() {
                for v in &$program.heap_variables {
                    if v.label == $label {variable = Some(v); break;}
                }
                for v in &$program.const_variables{
                    if v.label == $label {variable = Some(v); break;}
                }
                for v in &$program.static_variables{
                    if v.label == $label {variable = Some(v); break;}
                }
            }
            variable
        })
    }

    #[macro_export]
    macro_rules! grabVariableSetToken {
        ($tk: expr, $program: expr, $current_proc: expr) => ({
            let mut variable = match $current_proc{
                Some(p) => {
                    let mut fv = Default::default();
                    let mut len = $program.procs[p].arguments.len();
                    for i in 0..len{
                        let v = &$program.procs[p].arguments[i];
                        if v.label == $tk.tk_data {
                            fv = Some(v); 
                            $tk.tk_comp_data = TokenCompData::Var(i, VarDest::Argument);
                            break;
                        }
                    }
                    if fv == Default::default() {
                        len = $program.procs[p].variables.len();
                        for i in 0..len{
                            let v = &$program.procs[p].variables[i];
                            if v.label == $tk.tk_data {
                                fv = Some(v); 
                                $tk.tk_comp_data = TokenCompData::Var(i, VarDest::CurrentProc);
                                break;
                            }
                        }
                    }
                    fv
                }
                None => None
            };

            if variable.is_none() {
                let mut len = $program.heap_variables.len();
                for i in 0..len{
                    let v = &$program.heap_variables[i];
                    if v.label == $tk.tk_data {
                        variable = Some(v); 
                        $tk.tk_comp_data = TokenCompData::Var(i, VarDest::Heap);
                        break;
                    }
                }
                len = $program.const_variables.len();
                for i in 0..len{
                    let v = &$program.const_variables[i];
                    if v.label == $tk.tk_data {
                        variable = Some(v); 
                        $tk.tk_comp_data = TokenCompData::Var(i, VarDest::ProgramConst);
                        break;
                    }
                }
                len = $program.static_variables.len();
                for i in 0..len{
                    let v = &$program.static_variables[i];
                    if v.label == $tk.tk_data {
                        variable = Some(v); 
                        $tk.tk_comp_data = TokenCompData::Var(i, VarDest::ProgramStatic);
                        break;
                    }
                }
            }
            variable
        })
    }

    #[macro_export]
    macro_rules! declareVariable {
        ($current_var_def: expr, $dat: expr, $dvt: expr, $tk_iter: expr, $expr: expr, $program: expr, $current_proc: expr) => ({
            // check if expr open
            match $expr {
                Some(ref mut _exp) => return Err(ParserError::TypeInExpression($dvt)),
                None => {
                    // change tk
                    let mut vtk = $tk_iter.peek().unwrap();

                    let mut var_size: i16 = -1;
                    if ($dvt==DataValueType::Buffer) {
                        if vtk.tk_data == "[" {
                            $tk_iter.next();
                            var_size = match $tk_iter.next().unwrap().tk_data.parse::<i16>() {
                                Ok(s) => s,
                                Err(e) => return Err(ParserError::MissingSizeForBufferNotNumber)
                            };
                            if $tk_iter.next().unwrap().tk_data != "]" {
                                return Err(ParserError::MissingClosingSqBracket);
                            }
                            vtk = $tk_iter.peek().unwrap();
                        }
                        else {
                            return Err(ParserError::MissingClosingSqBracket);
                        }
                    }
                    
                    // identify, if already exists, throw error 
                    match grabVariable!(vtk.tk_data, $program, $current_proc){
                        Some(_v) => return Err(ParserError::RedefinitionOfVariable(vtk.tk_data.to_string())),
                        // let case for other label types and if none of them are matched throw error
                        None => (),
                    }

                    //declare var
                    match $current_proc {
                        Some(p) => {
                            //allocate bytes
                            let newDat = match $dat {
                                DataAllocationType::Stack(_) => {
                                    let s = program.procs[p].allocated_bytes;
                                    if var_size == -1 {
                                        program.procs[p].allocated_bytes += $dvt.size().ok_or(ParserError::UnimplementedDataValueType($dvt))?;
                                    } else {
                                        program.procs[p].allocated_bytes += var_size as u8;
                                    }
                                    DataAllocationType::Stack(s)
                                }
                                DataAllocationType::Const => DataAllocationType::Const,
                                _ => return Err(ParserError::UnimplementedDataAllocType($dat))
                            };

                            //declare var
                            $program.procs[p].variables.push(Variable {
                                t: DataType {
                                    a: newDat,
                                    v: $dvt
                                },
                                value: None,
                                label: vtk.tk_data
                            });
                            $current_var_def = Some(($program.procs[p].variables.len()-1, VarDest::CurrentProc));
                        }
                        None => {
                            //allocate bytes
                            let newDat = match $dat {
                                DataAllocationType::Heap(_) => {
                                    let s = program.allocated_bytes;
                                    program.allocated_bytes += $dvt.size().ok_or(ParserError::UnimplementedDataValueType($dvt))? as u16;
                                    DataAllocationType::Heap(s)
                                }
                                DataAllocationType::Const => {
                                    DataAllocationType::Const
                                }
                                DataAllocationType::Static => {
                                    DataAllocationType::Static
                                }
                                _ => return Err(ParserError::AttemptedVariableInProgram(vtk.tk_data.to_string()))
                            };

                            //declare var
                            match newDat{
                                DataAllocationType::Const => {
                                    $program.const_variables.push(Variable {
                                        t: DataType {
                                            a: newDat,
                                            v: $dvt
                                        },
                                        value: None,
                                        label: vtk.tk_data
                                    });
                                    $current_var_def = Some(($program.const_variables.len()-1, VarDest::ProgramConst));
                                }
                                DataAllocationType::Static => {
                                    $program.static_variables.push(Variable {
                                        t: DataType {
                                            a: newDat,
                                            v: $dvt
                                        },
                                        value: None,
                                        label: vtk.tk_data
                                    });
                                    $current_var_def = Some(($program.static_variables.len()-1, VarDest::ProgramStatic));
                                }
                                DataAllocationType::Heap(_) => {
                                    $program.heap_variables.push(Variable {
                                        t: DataType {
                                            a: newDat,
                                            v: $dvt
                                        },
                                        value: None,
                                        label: vtk.tk_data
                                    });
                                    $current_var_def = Some(($program.heap_variables.len()-1, VarDest::Heap));
                                } 
                                _ => return Err(ParserError::AttemptedVariableInProgram(vtk.tk_data.to_string()))
                            }
                        }
                    };
                }
            };
        })
    }

    #[macro_export]
    macro_rules! grabProcedure {
        ($label: expr, $program: expr) => ({
            let mut proc : Option<&Procedure<'_>> = None;
            for i in 0..$program.procs.len(){
                if $program.procs[i].label == $label {proc = Some(&$program.procs[i]); break;}
            }
            proc
        })
    }

    // put heap allocations on page 2
    program.allocated_bytes = 0x0200;
    let tks_len = token_storage.len();

    let mut current_proc: Option<usize> = None;
    let mut tk: & mut Token<'_>;
    let mut index: usize = 0;
    let mut tk_iter = token_storage.iter_mut().peekable();
    let mut expr: Option<Expression> = None;
    let mut nextDAT = DataAllocationType::Stack(0);
    let mut resolvableErrors: Vec<ParserError> = Default::default();
    let mut current_var_def: Option<(usize, VarDest)> = None;
    while tk_iter.len() != 0{
        tk=tk_iter.nth(0).unwrap();
        match tk.tk_type {
            TokenType::None => return Err(ParserError::UnidentifiedToken(tk.tk_data.to_string())),
            TokenType::KeywordProc => {
                let mut new_proc: Procedure<'a> = Default::default();

                // gather out type
                tk=tk_iter.next().unwrap(); //next token
                if tk.tk_data == ":" {
                    tk=tk_iter.next().unwrap(); //next token
                    let vt = match toValueType(&tk.tk_data) {
                        Ok(t) => t,
                        Err(e) => return Err(e)
                    };
                    new_proc.retType = DataType {a: DataAllocationType::None, v: vt};
                } else {
                    new_proc.retType = DataType {a: DataAllocationType::None, v: DataValueType::Void};
                }
                tk=tk_iter.next().unwrap(); //next token
                
                //set label
                new_proc.label = tk.tk_data;
                
                //get arguments
                tk=tk_iter.next().unwrap(); //next token
                if tk.tk_data != "(" {return Err(ParserError::NoParaProcDef);}
                tk=tk_iter.next().unwrap(); //next token
                while tk.tk_data != ")" && index < tks_len {
                    // make a var
                    let mut var: Variable<'a> = Default::default();
                    let vt = match toValueType(&tk.tk_data) {
                        Ok(t) => t,
                        Err(e) => return Err(e)
                    };
                    var.t = DataType { a: DataAllocationType::Stack(0), v: vt};
                    tk=tk_iter.next().unwrap(); //next token
                    var.label = tk.tk_data;
                    new_proc.arguments.push(var);

                    //inc
                    tk=tk_iter.next().unwrap(); //next token
                }
                if index >= tks_len-1 {return Err(ParserError::NoParaNotTerminated);}

                //set current proc
                if current_proc.is_some() {return Err(ParserError::MissingEndStatement);}
                program.procs.push(new_proc);
                current_proc = Some(program.procs.len()-1);
            }
            TokenType::KeywordEnd => {
                // TODO: Add support for blocks
                let _proc = match current_proc {
                    Some(p) => p,
                    None => return Err(ParserError::ExcessiveEndStatement)
                };
                current_proc = None;
            }
            TokenType::KeywordRet => {
                //start expression
                if expr.is_some() {return Err(ParserError::SymbolIncorrectlyInExpression);}
                let mut built_expr: Expression = Default::default();
                built_expr.t = ExpressionType::Return;
                
                expr = Some(built_expr);
            }
            TokenType::SymbolSemicolon => {
                let mut pushExpr: bool = true; 
                let unpkg_expr = match expr {
                    Some(ref exp) => exp,
                    None => return Err(ParserError::UnnecessarySemicolon)
                };
                match current_proc {
                    Some(p) => {
                        let exp = unpkg_expr.clone();
                        if exp.tks.len() == 1 {
                            match exp.t {
                                ExpressionType::Unspecified =>{
                                    match exp.tks[0].tk_type{
                                        TokenType::Variable => {
                                            warnings.push(ParserWarning::WarningNoInitialValue(exp.tks[0].tk_data.to_string()));
                                            pushExpr = false;
                                        },
                                        _ => warnings.push(ParserWarning::WarningUnidentified)
                                    }
                                }
                                _ => ()
                            }
                        }
                        else {
                            match exp.t {
                                ExpressionType::Assignment =>{
                                    match exp.tks[0].tk_type{
                                        TokenType::Variable => {
                                            let unpkged_var = match current_var_def {
                                                Some(i) => {
                                                    match i.1 {
                                                        VarDest::CurrentProc => {
                                                            & mut program.procs[{match current_proc {
                                                                Some(p) => p,
                                                                None => return Err(ParserError::AttemptedVariableInProgram(exp.tks[0].tk_data.to_string()))
                                                            }}].variables[i.0]
                                                        }
                                                        VarDest::Heap => & mut program.heap_variables[i.0],
                                                        VarDest::ProgramConst => & mut program.const_variables[i.0],
                                                        _ => return Err(ParserError::AttemptedVariableInProgram(exp.tks[0].tk_data.to_string()))
                                                    }
                                                },
                                                None => return Err(ParserError::UnidentifiedError)
                                            };
                                            // only supoorts `const TYPE = VALUE;` 
                                            if unpkged_var.t.a == DataAllocationType::Const {
                                                unpkged_var.value = Some(exp.tks[2]);
                                                // ignores other resolvables, add a macro called "resolve!(e)"
                                                resolvableErrors.pop();
                                                pushExpr = false;
                                            }
                                        }
                                        _ => ()
                                    }
                                }
                                _ => ()
                            }
                        }
                        if pushExpr{
                            let len = program.procs[p].expressions.len();
                            program.procs[p].lines.push(Line {index: len, t: LineType::Expression});
                            program.procs[p].expressions.push(unpkg_expr.clone());
                        }
                    }
                    None => {
                        let exp = unpkg_expr.clone();
                        match exp.t {
                            ExpressionType::Assignment => {
                                match exp.tks[0].tk_type{
                                    TokenType::Variable => {
                                        let unpkged_var = match current_var_def {
                                            Some(i) => {
                                                match i.1 {
                                                    VarDest::Heap => & mut program.heap_variables[i.0],
                                                    VarDest::ProgramConst => & mut program.const_variables[i.0],
                                                    VarDest::ProgramStatic => & mut program.static_variables[i.0],
                                                    _ => return Err(ParserError::AttemptedVariableInProgram(exp.tks[0].tk_data.to_string()))
                                                }
                                            }
                                            None => return Err(ParserError::UnidentifiedError)
                                        };
                                        // only supoorts `const TYPE = VALUE;` 
                                        if unpkged_var.t.a == DataAllocationType::Const {
                                            unpkged_var.value = Some(exp.tks[2]);
                                            // ignores other resolvables, add a macro called "resolve!(e)"
                                            resolvableErrors.pop();
                                            pushExpr = false;
                                        }
                                    }
                                    _ => return Err(ParserError::AttemptedExpressionInProgram)
                                }
                            }
                            _ => return Err(ParserError::AttemptedExpressionInProgram)
                        }
                        if pushExpr{
                            program.expressions.push(unpkg_expr.clone());
                        }
                    }
                };
                expr = None;
                if resolvableErrors.len() != 0 {
                    return Err(resolvableErrors[0].clone());
                }
            }
            TokenType::KeywordUint => {
                declareVariable!(current_var_def, nextDAT, DataValueType::Uint, tk_iter, expr, program, current_proc);
                nextDAT = DataAllocationType::Stack(0);
            }
            TokenType::KeywordShort => {
                declareVariable!(current_var_def, nextDAT, DataValueType::Short, tk_iter, expr, program, current_proc);
                nextDAT = DataAllocationType::Stack(0);
            }
            TokenType::KeywordString => {
                declareVariable!(current_var_def, nextDAT, DataValueType::String, tk_iter, expr, program, current_proc);
                nextDAT = DataAllocationType::Stack(0);
            }
            TokenType::KeywordBuffer => {
                declareVariable!(current_var_def, nextDAT, DataValueType::Buffer, tk_iter, expr, program, current_proc);
                nextDAT = DataAllocationType::Stack(0);
            }
            TokenType::KeywordConst => {
                nextDAT = DataAllocationType::Const;
            }
            TokenType::KeywordHeap => {
                nextDAT = DataAllocationType::Heap(0);
            }
            TokenType::KeywordStatic => {
                nextDAT = DataAllocationType::Static;
            }
            TokenType::OpAssign => {
                match expr {
                    Some(ref mut exp) =>{
                        exp.t = ExpressionType::Assignment;
                        exp.tks.push(tk);
                    }
                    None => return Err(ParserError::StrayAssignment)
                };
            }
            TokenType::OpAdd |
            TokenType::OpSubtract => {
                match expr {
                    Some(ref mut exp) => exp.tks.push(tk),
                    None => return Err(ParserError::StrayOperator(tk.tk_data.to_string()))
                };
            }
            TokenType::Symbol |
            TokenType::CharLiteral |
            TokenType::StringLiteral |
            TokenType::NumberLiteral |
            TokenType::HexNumberLiteral => {
                match expr {
                    Some(ref mut exp) => exp.tks.push(tk),
                    None => return Err(ParserError::StrayValue(tk.tk_data.to_string()))
                };
            }
            TokenType::EmbeddedFunction |
            TokenType::Register => {
                // write identify method in proc to do this
                match expr {
                    Some(ref mut exp) => exp.tks.push(tk),
                    None => {
                        let mut built_expr: Expression = Default::default();
                        built_expr.t = ExpressionType::Unspecified;
                        built_expr.tks.push(tk);
                        
                        expr = Some(built_expr);
                    }
                };
            }
            TokenType::UnidentifiedLabel => {
                // identify
                let variable = match grabVariableSetToken!(tk, program, current_proc){
                    Some(v) => {
                        tk.tk_type = TokenType::Variable;
                        if v.t.a == DataAllocationType::Const {
                            match v.value {
                                Some(ref value_tk) => {
                                    tk.tk_type = value_tk.tk_type;
                                    tk.tk_data = value_tk.tk_data;
                                }
                                None => resolvableErrors.push(ParserError::ConstNoInitial(tk.tk_data.to_string()))
                            }
                        }
                    }
                    // let case for other label types and if none of them are matched throw error
                    None => {
                        match grabProcedure!(tk.tk_data, program){
                            Some(ref p) => {
                                tk.tk_type = TokenType::ProcedureCall;
                            }
                            None => return Err(ParserError::UnidentifiedLabel(tk.tk_data.to_string()))
                        }
                    }
                };
                
                // write identify method in proc to do this
                match expr {
                    Some(ref mut exp) => exp.tks.push(tk),
                    None => {
                        let mut built_expr: Expression = Default::default();
                        built_expr.t = ExpressionType::Unspecified;
                        built_expr.tks.push(tk);
                        
                        expr = Some(built_expr);
                    }
                };
            }
            _ => println!("Case not covered: {tk:?}"),
        }
        index+=1;
    }
    if current_proc.is_some() {return Err(ParserError::MissingEndStatement);}
    return Ok(program);
}