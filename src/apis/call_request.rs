use crate::models::general::llm::{Message,ChatCompletion};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

//call model 

pub async fn call_gpt(messages: Vec<Message>){
  dotenv().ok();

  //extract api key
  let api_key:String = env::var("OPEN_AI_KEY").expect("Key not found in environment variables");
  let api_org:String = env::var("OPEN_AI_ORG").expect("Key not found in environment variables");


  //confirm endpoint

  let url:&str = "https://api.openai.com/v1/chat/completions";

  //create headers

  let mut headers: HeaderMap = HeaderMap::new();


  //create api key header
  headers.insert("authorization", HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap()
);

  //create open ai org header
  headers.insert("OpenAI-Organization", HeaderValue::from_str(api_org.as_str()).unwrap());


  //client

  let client = Client::builder().default_headers(headers).build().unwrap();

  //create chat completion

  let chat_completion:ChatCompletion = ChatCompletion { model: "gpt-3.5-turbo".to_string(), messages , temperature: 0.1 };

  //trouble shoot

  let res_raw = client.post(url).json(&chat_completion).send().await.unwrap();

  dbg!(res_raw.text().await.unwrap());

}