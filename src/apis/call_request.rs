use crate::models::general::llm::Message;
use dotenv::dotenv;
use reqwest::Client;
use std::env;

//call model 

pub async fn call_gpt(message: Vec<Message>){
  dotenv().ok();

  //extract api key
  let api_key:String = env::var("OPEN_AI_KEY").expect("Key not found in environment variables");
  let api_org:String = env::var("OPEN_AI_ORG").expect("Key not found in environment variables");


  //confirm endpoint

  let url:&str = "https://api.openai.com/v1/chat/completions";


}