#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use crate::lexer::token_type::*;


#[derive(Debug)]
pub enum BinaryExpressionType{
    Assignment_op,
    Addition_op,
    Subtraction_op,
    Multiplication_op,
    Division_op,
}


pub struct BinaryExpression {

    pub exp_value : Option<Token> ,
    pub exp_type : Option<BinaryExpressionType>,
    pub exp_left : Option<Token>,
    pub exp_right :Option<Token> ,
}


pub struct BinaryExpressionTree{
    pub tree : Vec<BinaryExpression>,
}


