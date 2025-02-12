#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

use crate::binaryexp::BinaryExpressionBlock;
use crate::lexer::token_type::Token;

pub struct Scope{
    scope : Vec<String>,
}
#[derive(Debug)]
pub enum VAR_Definition_Type{
    def_with_type,
    def_with_type_as_arg,
    def_with_type_value,
    def_with_infered_value,
}

#[derive(Debug)]
pub struct BLOCK{
    block : Vec<Expression>
}


#[derive(Debug)]
pub struct VAR_Definition{
    def_type : VAR_Definition_Type ,
    data_type : Token,
    data_id : Token,
    data_value : Token,
}


#[derive(Debug)]
pub struct FN_Definition{
    fn_id : Token ,
    fn_args : Vec<VAR_Definition> ,
    fn_body : BLOCK,
}

#[derive(Debug)]
pub enum Expression{
    binary_exp(BinaryExpressionTree),
    var_definition(VAR_Definition),
    fn_definition(FN_Definition),
//    if_statement(IF_Statement),
//    else_if_statement(ELSE_IF_Statement),
//    else_statement(ELSE_Statement),
//    for_statement(FOR_Statement),
    scope_block(BLOCK),
}
