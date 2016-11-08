use commands::command::*;

pub fn run_init_command() -> CommandResult {
  println!("Init command!");
  command_result!(true)
}
