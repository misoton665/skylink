use std::io;

use commands::command::*;

pub fn run_init_command() -> CommandResult {
  println!("Init command will clean configure file, and make new empty one.\nDo you really run init command? (y/n): ");

  loop {
    let mut answer = String::new();

    match io::stdin().read_line(&mut answer) {
      Ok(_) => (),
      Err(_) => {
        println!("IO error.");
        continue;
      }
    };

    let answer = answer.trim();

    if answer == "n" {
      return command_result!(false);
    } else if answer == "y" {
      break;
    } else {
      println!("Please enter the character \'y\' or \'n\'");
    }
  }
  command_result!(true)
}
