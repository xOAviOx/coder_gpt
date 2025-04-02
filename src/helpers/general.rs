use std::fmt::format;
use crate::models::general::llm::Message;

//extend ai func to allow specific output


pub fn extend_ai_function(ai_func: fn(&str) -> &'static str,func_input: &str) -> Message {
  let ai_function_str = ai_func(func_input);
  // dbg!(ai_function_str);

  //extend the string to allow only printing the output

  let msg : String = format!("FUNCITON: {}
  INSTRUCTION: You are a function printer. You ONLY print the results of functions. Nothing else. No commentary. Here is the input to the function: {}. 
  Print out what the function will return.", ai_function_str, func_input);

  //return message

  Message {
    role: "system".to_string(),
    content:msg
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
  #[test]

  fn tests_extending_ai_functions() { 
    let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy variable");
    dbg!(&extended_msg);
    assert_eq!(extended_msg.role, "system".to_string());
  }
}