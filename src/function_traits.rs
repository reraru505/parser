use crate::function::*;


impl Clone for FunctionDefType{
    fn clone(&self ) -> Self {
	match self {
	    FunctionDefType::DEF_WITH_ARGS =>  FunctionDefType::DEF_WITH_ARGS,          
	    FunctionDefType::DEF_WTTH_ARGS_NO_RETURN => FunctionDefType::DEF_WTTH_ARGS_NO_RETURN,
	    FunctionDefType::DEF_NO_ARGS =>  FunctionDefType::DEF_NO_ARGS,            
	    FunctionDefType::DEF_NO_ARGS_NO_RETURN => FunctionDefType::DEF_NO_ARGS_NO_RETURN,  
	}
    }
}

impl Clone for FunctionDef{

    fn clone(&self ) -> Self {

	Self {
	    fn_id : self.fn_id.clone(),
	    fn_type : self.fn_type.clone(),
	    fn_return_type : self.fn_return_type.clone(),
	    fn_args : self.fn_args.clone(),
	    fn_body : None,
	}
	
    }
    
}
