use std::fmt::format;
use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Deserialize};

use crate::{apis::call_request::call_gpt, models::general::llm::{self, Message}};

use super::command_line::PrintCommand;

const CODE_TEMPLATE_PATH : &str = "C:/Users/avish/Desktop/web-server-template-main/src/code_template.rs";
const EXEC_MAIN_PATH : &str = "C:/Users/avish/Desktop/web-server-template-main/src/main.rs";
const API_SCHEMA_PATH : &str = "C:/Users/avish/Desktop/coder_gpt/schemas/api_schema.json";



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


//performs call to llm
pub async fn ai_task_request  (
  msg_context: String,
  agent_position: &str,
  agent_operation: &str,
  function_pass: for<'a> fn(&'a str)->&'static str,

)->String {

  //extend ai functions
  let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

  //print current status
  PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

  //Get llm response
  let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> = call_gpt(vec![extended_msg.clone()]).await;

  //return success or try again
   match llm_response_res {
      Ok(llm_resp)=>llm_resp,

      Err(_) => call_gpt(vec![extended_msg.clone()]).await.expect("Failed twice to call OpenAi")
  }

}



//performs call to llm - decoded
pub async fn ai_task_request_decoded<T: DeserializeOwned>  (
  msg_context: String,
  agent_position: &str,
  agent_operation: &str,
  function_pass: for<'a> fn(&'a str)->&'static str,

)-> T {


  let llm_response:String = ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

  let decoded_response: T = serde_json::from_str(llm_response.as_str()).expect("Failed to decode ai response from serde_json");

  return decoded_response;

}


// check whether request url is valid
pub async fn check_status_code(client:&Client, url:&str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}


// get code template




// Save new backend code




//save json api endpoint schema



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


  #[tokio::test]
  async fn tests_ai_task_request() {

      let ai_fucn_param: String = "Build me a webserver for making stock price api requests.".to_string();
      let res = ai_task_request(ai_fucn_param, "Managing Agent", "Defining user requirements", convert_user_input_to_goal).await;

      assert!(res.len()>20);
  }
}

