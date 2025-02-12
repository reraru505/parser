//use std::rc::Rc;
//use std::cell::RefCell;
//
//
//use crate::lexer::token_type::*;
//use crate::expressions::*;
//use std::process;
//
//pub struct binary_exp_checker;
//
//impl binary_exp_checker{
//
////    pub fn find(context : Rc<RefCell<Vec<Token>>>) -> Expression{
////
////	
////	
////    }
//
//    pub fn break_exp(&self , context : Rc<RefCell<Vec<Token>>>) -> Binary_Expression{
//
//	let len = context.borrow().len();
//	let mut index : usize = 0;
//
//	let mut root_bin_exp = Binary_Expression::new();
//	let mut leaf_bin_exp = &mut root_bin_exp;
//	
//	while  index < len {
//
//	    match context.borrow()[index] {
//		Token::t_operator(_) => {
//
//		    if let Some ( a) = self.find_binary_exp_type(context.borrow()[index].clone()){
//			leaf_bin_exp.exp_type = Some(a)
//		    };
//		    leaf_bin_exp.exp_left = Some(context.borrow()[index - 1].clone());
//		    leaf_bin_exp.exp_right = Some(Box::new(Binary_Expression::new()));
//		    if let Some(a) = &leaf_bin_exp.exp_right{
//			let leaf_bin_exp = &a;
//		    };
//		    index += 1;
//		}
//		_ => {
//		    
//		    if index == (len - 2) {
//			leaf_bin_exp.exp_left = Some(context.borrow()[index - 1].clone());
//			break;
//		    }
//		    index += 1;
//		}
//	    }
//	    
//	    
//	} 
//
//	return root_bin_exp.clone();
//    }
//
//    fn find_binary_exp_type(&self , tok : Token ) -> Option<Binary_Expression_Type>{
//
//	match tok {
//
//	    Token::t_operator(Operator::assignment_op(_)) => return Some(Binary_Expression_Type::assignment_op),
//	    Token::t_operator(Operator::addition_op(_)) => return Some(Binary_Expression_Type::addition_op),
//	    Token::t_operator(Operator::subtraction_op(_)) => return Some(Binary_Expression_Type::subtraction_op),
//	    Token::t_operator(Operator::multiplication_op(_)) => return Some(Binary_Expression_Type::multiplication_op),
//	    Token::t_operator(Operator::division_op(_)) => return Some(Binary_Expression_Type::division_op),
//	    _ => return None,
//	}
//	
//    }
//    
//}
//
