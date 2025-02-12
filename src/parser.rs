use crate::expressions::*;
use crate::lexer::lex::Lexeme;

struct Parser{
    lexemes : Vec<Lexeme>,
    expressions : Vec<Expression>,
}


impl Parser{
    pub fn new(in_lexemes : Vec<Lexeme>) -> Self{

	Self{
	    lexemes : in_lexemes,
	    expressions : Vec::new(),
	}
	
    }

    pub fn parse(&self){

	let lex_len = self.lexemes.len();
	let context : Vec<Lexeme> = Vec::new();

	let mut index : usize = 0;

	while index < lex_len {

	    
	    
	}
	
    }
}
