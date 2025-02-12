use std::rc::Rc;
use std::cell::RefCell;

extern crate lexer;

//pub mod expressions;
//pub mod parser;
//pub mod grammer;

pub mod binaryexp;
pub mod binaryexp_impl;
pub mod binaryexp_helpers;

use crate::lexer::lex::Lexeme;
use crate::lexer::token_type::Token;
//use crate::grammer::*;
use crate::binaryexp::*;
use crate::binaryexp_impl::*;

pub fn parse(inp_lexemes : Vec<Lexeme> ){

    
    let mut tokens : Vec<Token> = Vec::new();
    for i in inp_lexemes{
	tokens.push(i.tokens);
    }

    break_binary_expression(&mut tokens);
    println!("WHyyyy");
   
}
