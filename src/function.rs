use crate::parsingdata::ParsingData;
use crate::lexer::token_type::*;
use crate::expressions::*;
use crate::symboltable::symbol::DataType;
use crate::lexer::lex::Lexeme;

#[derive(Debug)]
pub enum FunctionDefType{

    DEF_WITH_ARGS,
    DEF_WTTH_ARGS_NO_RETURN,
    DEF_NO_ARGS,
    DEF_NO_ARGS_NO_RETURN,
    
}

#[derive(Debug)]
pub struct FunctionDef{

    pub fn_id : Token ,
    pub fn_type : Option<FunctionDefType>,
    pub fn_return_type : DataType,
    pub fn_args : Option< Vec<Lexeme> >,
    pub fn_body : Option<Block>,
}

impl FunctionDef{
    pub fn new(fid : Token , frt : DataType ) -> Self{
	Self{
	    fn_id : fid ,
	    fn_type : None,
	    fn_return_type : frt,
	    fn_args : None,
	    fn_body : None,
	}
    }
}


pub fn find_all_function_def(parsingvec : Vec<ParsingData>) -> Vec<ParsingData>{

    let mut context : Vec<ParsingData> = Vec::new();
    let mut retval  : Vec<ParsingData> = Vec::new();
    
    for i in 0.. parsingvec.len(){

	if i > 3{
	        context = parsingvec[i-4 .. i].to_vec();
	}

	if is_function_def(context.clone()){

	    for _ in 0..4{
		retval.pop();
	    }
	    retval.push(ParsingData::functiondef(FunctionDef::new(get_function_id(context.clone()),
								  get_function_return_type(context.clone()))));
	    context.clear();
	}
	retval.push(parsingvec[i].clone());
	

	
    }
    
    return retval;
}

pub fn get_function_return_type(in_context : Vec<ParsingData>) -> DataType{

    if let ParsingData::lexeme(s) = in_context[2].clone() {
	match s.tokens {
	    
	    Token::t_keyword(Keyword::data_type(Data_Type::CHAR(_))) => return DataType::CHAR,
	    Token::t_keyword(Keyword::data_type(Data_Type::I32(_))) => return DataType::I32,
	    Token::t_keyword(Keyword::data_type(Data_Type::I64(_))) => return DataType::I64,
	    Token::t_keyword(Keyword::data_type(Data_Type::F32(_))) => return DataType::F32,
	    Token::t_keyword(Keyword::data_type(Data_Type::F64(_))) => return DataType::F64,
	    Token::t_keyword(Keyword::data_type(Data_Type::STRING(_)))  => return DataType::STRING,
	    Token::t_keyword(Keyword::data_type(Data_Type::VOID(_))) => return DataType::VOID,
	    _ => return DataType::VOID, //this case will not be possible but the rust compiler is a bitch
	}
    }
    return DataType::VOID;//again rust compiler is a bitch
}

pub fn get_function_id(in_context : Vec<ParsingData>) -> Token{

    if let ParsingData::lexeme(s) = in_context[0].clone(){
	return s.tokens;
    }
    return Token::t_identifier("THIS_CASE_CAN_NOT_OCCUR".to_string());
}

pub fn is_function_def(in_context : Vec<ParsingData>) -> bool{

    if in_context.len() < 4 {
	return false;
    }

    let mut retval = false;

    for i in 0..4 {
	if let ParsingData::lexeme(s) = in_context[i].clone(){
	    match i{
		0 => if matches!(s.tokens , Token::t_identifier(_)){retval = true;}
		else{return false; },
		1 => if matches!(s.tokens , Token::t_operator(Operator::type_assignment_op(_))){retval = true;}
		else{return false; },
		2 => if matches!(s.tokens , Token::t_keyword(Keyword::data_type(_))){retval = true;}
		else{return false; },
		3 => if matches!(s.tokens , Token::t_keyword(Keyword::statement(Statement::function_marker(_)))){retval = true;}
		else{return false; },
		_ => return false,
	    }
	    
	} 
    }
    

    return retval;
    
}

//pub fn drain_args(context : &mut Vec<ParsingData>) {
//
//    let mut inner_brackets = 0;
//    let mut start = 0;
//    let mut end = 0;
//    
//    for (index , i) in context.clone().iter().enumerate(){
//	if matches!(i.tokens , Token::t_stc(STC::stc_arg_begin(_))){
//	    
//	}
//    }
//    
//}
