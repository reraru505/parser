
use crate::binaryexp::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use crate::lexer::token_type::*;


impl BinaryExpressionType{
    pub fn get_highest_precedence(context : Vec<Token>) -> usize {

	let mut precedence : usize  = 0;

	for i in context.iter(){
	    match i {
		Token::t_operator(Operator::addition_op(s)) | Token::t_operator(Operator::subtraction_op(s)) =>  {
		    precedence = precedence.max(2);
		} ,
		Token::t_operator(Operator::multiplication_op(s)) | Token::t_operator(Operator::division_op(s)) =>  {
		    precedence = precedence.max(3);
		},
		Token::t_stc(STC::stc_arg_begin(s)) =>{
		    precedence = precedence.max(4);
		},
		Token::t_operator(Operator::assignment_op(s)) =>{
		    precedence = precedence.max(1);
		},
		_ => {
		    precedence = precedence.max(0);
		},
	    };
	}

	return precedence;
    }

    pub fn check_precedence(token :Token) -> usize{
	match token{
	    Token::t_operator(Operator::addition_op(s)) | Token::t_operator(Operator::subtraction_op(s)) => return 2,
	    Token::t_operator(Operator::multiplication_op(s)) | Token::t_operator(Operator::division_op(s)) => return 3,
	    Token::t_operator(Operator::assignment_op(s)) => return 1,
	    _ => return 0,
	}
    }

    pub fn get_binary_expression_type(token :Option<Token>) -> BinaryExpressionType{
	
	match token{
	    Some(Token::t_operator(Operator::assignment_op(_))) => return BinaryExpressionType::Assignment_op,
	    Some(Token::t_operator(Operator::addition_op(_))) => return BinaryExpressionType::Addition_op,
	    Some(Token::t_operator(Operator::subtraction_op(_))) => return BinaryExpressionType::Subtraction_op,
	    Some(Token::t_operator(Operator::division_op(_))) => return BinaryExpressionType::Division_op,
	    Some(Token::t_operator(Operator::multiplication_op(_))) => return BinaryExpressionType::Multiplication_op,
	    _ => return BinaryExpressionType::Assignment_op,
	}
	
    }

    pub fn is_binary_operator(token : Token ) -> bool{
	
	match token{
	    Token::t_operator(Operator::assignment_op(_)) => return true,
	    Token::t_operator(Operator::addition_op(_)) => return true,
	    Token::t_operator(Operator::subtraction_op(_)) => return true,
	    Token::t_operator(Operator::division_op(_)) => return true,
	    Token::t_operator(Operator::multiplication_op(_)) => return true,
	    _ => return false,
	}
	
    }
}

impl fmt::Display for BinaryExpressionType{

    fn fmt(&self ,  f: &mut fmt::Formatter<'_>) -> fmt::Result{
	match self{
	    BinaryExpressionType::Assignment_op => write!(f, " = ", ),
	    BinaryExpressionType::Addition_op => write!(f, " + ", ),
	    BinaryExpressionType::Subtraction_op => write!(f, " - ", ),
	    BinaryExpressionType::Multiplication_op => write!(f, " * ", ),
	    BinaryExpressionType::Division_op => write!(f, " / ", ),
	}
    }
    
}

impl Clone for BinaryExpressionType{
    fn clone(&self ) -> Self{
	match self{
	    BinaryExpressionType::Assignment_op => BinaryExpressionType::Assignment_op,
	    BinaryExpressionType::Addition_op => BinaryExpressionType::Addition_op,
	    BinaryExpressionType::Subtraction_op => BinaryExpressionType::Subtraction_op,
	    BinaryExpressionType::Multiplication_op => BinaryExpressionType::Multiplication_op,
	    BinaryExpressionType::Division_op => BinaryExpressionType::Division_op,
	}
    }
}

impl BinaryExpression{

    pub fn new() -> Self {
	Self{
	    
	    exp_value : None,
	    exp_type :  None,
	    exp_left :  None,
	    exp_right : None,
	}
    }
}

impl BinaryExpressionTree{
    pub fn new() -> Self{
	Self{
	    tree : Vec::new(),
	}
    }

    pub fn maketree(&mut self , tree_maker : Vec<Option<Token>> ){

	let len = tree_maker.len();
	let mut index : usize  = 0;
	while index < len{

	    self.tree.push(BinaryExpression{
		exp_value : tree_maker[index].clone(),
		exp_type :  {if let Some(_) = tree_maker[index + 1].clone() {
		    Some(BinaryExpressionType::get_binary_expression_type(tree_maker[index + 1].clone()))
		}else {
		    None
		}},
		exp_left :  tree_maker[index + 2].clone(),
		exp_right :  tree_maker[index + 3].clone(),}
	    );

	    index += 4;
	}
	
    }

    

    
    
}


pub fn handle_outside_brackets(context : Vec<Token> ,replacer : Token) -> Vec<Token>{

    let mut ncon = context.clone();
    let mut start = 0;
    
    for (index , i) in ncon.iter().enumerate() {
	if matches!(i , Token::t_stc(STC::stc_arg_begin(_))){
	    start = index ;
	    break;
	} 
    }

    let mut inner = 0;
    let mut end = 0;
    let mut index = start + 1;
    let len = ncon.len().clone();
    
    while index < len {
	
	if matches!(ncon[index] , Token::t_stc(STC::stc_arg_begin(_))){
	    inner += 1;
	}else if matches!(ncon[index] , Token::t_stc(STC::stc_arg_end(_))){
	    if inner == 0{
		end = index;
		break;
	    }else{
		inner -= 1;
	    }
	   
	}
	index +=1 ;
    }

    ncon[start] = replacer;
    ncon.drain(start+1 .. end + 1);

   // for i in ncon.clone(){
   // 	println!("outer = {:?}", i);
   // }

    println!("\n");

    return ncon;
    
}


pub fn handle_inside_brackets(context : Vec<Token> ) -> Vec<Token>{

    
    let mut ncon = context.clone();
    let mut start = 0;
    
    for (index , i) in ncon.iter().enumerate() {
	if matches!(i , Token::t_stc(STC::stc_arg_begin(_))){
	    start = index ;
	    break;
	} 
    }

    let mut inner = 0;
    let mut end = 0;
    let mut index = start + 1;
    let len = ncon.len().clone();
    
    while index < len {
	
	if matches!(ncon[index] , Token::t_stc(STC::stc_arg_begin(_))){
	    inner += 1;
	}else if matches!(ncon[index] , Token::t_stc(STC::stc_arg_end(_))){
	    if inner == 0{
		end = index;
		break;
	    }else{
		inner -= 1;
	    }
	}
	index +=1 ;
    }

    let retval = ncon[start + 1 .. end ].to_vec();

  //  for i in retval.clone(){
  //  	println!("inner = {:?}", i);
  //  }


    return retval;
    
}



pub fn remove_brackets_from_single_token_inside_brackets(context : Rc<RefCell<Vec<Token>>>) ->Vec<Token>{

    let new_context = context.borrow().clone();
    let mut replacer : Vec<Token> = Vec::new();
    let len = new_context.len();
    let mut index = 0;
    println!("\n\nskip_single_token_inside_brackets Starts\n\n");

    while index < len {

	if index < (len - 2) &&
	    matches!(new_context[index] , Token::t_stc(STC::stc_arg_begin(_))) &&
	    matches!(new_context[index + 2] ,Token::t_stc(STC::stc_arg_end(_))){
		index += 1;
		replacer.push(new_context[index].clone());
		index += 2;
	}else {
		
		replacer.push(new_context[index].clone());
		index += 1;
	    }	
    }
    println!("\n\nskip_single_token_inside_brackets Ends\n\n");
    return replacer;
    
}

pub fn print_binary_expression_tree_debug(tree : BinaryExpressionTree){
    
    let mut printName = String::new();
    let mut printRight = String::new();
    let mut printLeft = String::new();
    let mut printOp = String::new();
    for i in tree.tree.iter(){
	if let Some(Token::t_identifier(s)) = i.exp_value.clone(){
	    printName = s;
	}else{
	    printName.clear();
	}
	if let Some(Token::t_identifier(s)) = i.exp_left.clone(){
	    printLeft = s;
	}else if let Some(Token::t_literal(Literal::integer_literal(s))) = i.exp_left.clone(){
	    printLeft = s;
	}else {
	    printLeft.clear();
	}
	if let Some(Token::t_identifier(s)) = i.exp_right.clone(){
	    printRight = s;
	}else if let Some(Token::t_literal(Literal::integer_literal(s))) = i.exp_right.clone(){
	    printRight = s;
	}else{
	    printRight.clear();
	}if let Some(b) = i.exp_type.clone(){
	    printOp = format!("{}",b);
	}else{
	    printOp.clear();
	}
	println!("let {} = {} {} {} ;" , printName , printLeft , printOp , printRight);
    }

}
