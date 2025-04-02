use crossterm::{
  style::{Color, ResetColor, SetForegroundColor},
  ExecutableCommand
};
use std::io::{stdin,stdout};


//get user request
pub fn get_user_response(question: &str) -> String {
  let mut stdout: std::io::Stdout = stdout();

  //print the question in a specific color

  stdout.execute(SetForegroundColor(Color::Blue)).unwrap();

  println!(""); 

  println!("{}", question);

  //reset the color

  stdout.execute(ResetColor).unwrap();

  //read user input

  let mut user_response: String = String::new();

  stdin().read_line(&mut user_response).expect("Failed to read the response");

  //trim whitespace and return

  return user_response.trim().to_string()
}