use crossterm::{
  style::{Color, ResetColor, SetForegroundColor},
  ExecutableCommand
};
use std::io::{stdin,stdout, Stdout};

#[derive(Debug, PartialEq)]

pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos : &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        //decide on the print color

        let statement_color:Color = match self{
          Self::AICall => Color::Cyan,
          Self::UnitTest => Color::Magenta,
          Self::Issue => Color::Red,
        };

        //priont the agent statement

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("Agent: {}: ", agent_pos);


        //make slected color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        //Reset color

        stdout.execute(ResetColor).unwrap();
    }
}


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


#[cfg(test)]

mod tests {
  use super::*;

  #[test]
  fn tests_print_agent_msg(){
    PrintCommand::AICall.print_agent_message("Managing Agent", "Testing testing, processing something");
  }
}