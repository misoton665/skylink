use commands::command::*;

pub fn run_add_command(args: CommandArgs) -> CommandResult {
  println!("Add command!");
  command_result!(true)
}
