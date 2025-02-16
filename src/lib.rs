use std::rc::Rc;
use std::cell::RefCell;

extern crate lexer;
extern crate symboltable;

pub mod expressions;
pub mod parsingdata;

pub mod parser;


pub mod function;
pub mod function_traits;
pub mod function_args;

pub mod variable;

pub mod binaryexp;
pub mod binaryexp_impl;
pub mod binaryexp_helpers;
pub mod binaryexp_traits;
pub mod binaryexp_handle;

use crate::lexer::lex::Lexeme;
//use crate::lexer::token_type::Token;
//use crate::grammer::*;
//use crate::binaryexp::*;
//use crate::binaryexp_handle::*;
use crate::parsingdata::ParsingData;
use crate::function::{find_all_function_def ,
		      find_all_function_args ,
		      find_all_function_blocks ,
		      hanble_function_args};

pub fn parse(inp_lexemes : Vec<Lexeme> ){

    //let grammer = Grammer;
    //let ret = grammer.is_valid_binary_expression(inp_lexemes);
    //println!("{}", ret);
    
    // let mut tokens : Vec<Token> = Vec::new();
    // for i in inp_lexemes{
    //	tokens.push(i.tokens);
    // }
    //
    // break_binary_expression(&mut tokens);
    // println!("WHyyyy");

    let parsingdatavec = ParsingData::generate(inp_lexemes);
    let retval = find_all_function_def(parsingdatavec);
    let retval = find_all_function_args(retval);
    let retval = find_all_function_blocks(retval);
    let retval = hanble_function_args(retval);
    for i in retval{
	println!("{:#?}\n" , i);
    }
    
    
}
