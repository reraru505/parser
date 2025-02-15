//use crate::expressions::*;
//use crate::lexer::token_type::*;

use crate::function::FunctionDef;
use crate::lexer::lex::Lexeme;
use crate::binaryexp::BinaryExpressionBlock;

struct Parser;

#[derive(Debug)]
pub enum ParsingData{
    lexeme( Lexeme)  ,
    binexp( BinaryExpressionBlock),
    functiondef (FunctionDef),
    temp_arg_indicator,
}


impl ParsingData{
    pub fn generate(in_lexemes : Vec<Lexeme>) -> Vec<ParsingData>{
	let mut retval : Vec<ParsingData> = Vec::new();

	for i in in_lexemes.iter(){
	    retval.push(ParsingData::lexeme(i.clone()));
      	}

	return retval;
    }
}


impl Clone for ParsingData{

    fn clone(&self ) -> Self{
	match self {
	    ParsingData::lexeme(s) => return ParsingData::lexeme(s.clone()),
	    ParsingData::binexp(s) => return ParsingData::binexp(s.clone()),
	    ParsingData::functiondef(s) => return ParsingData::functiondef(s.clone()),
	    ParsingData::temp_arg_indicator => return ParsingData::temp_arg_indicator,
	}
	
    }
    
}
