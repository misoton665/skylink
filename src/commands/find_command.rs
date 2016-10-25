use commands::command::*;

pub fn run_find_command(args: CommandArgs) -> CommandResult {
  println!("find command");
  if args.len() == 0 {
    println!("find command was called, but argument is nothing!!!!!");
    command_result!(false)
  } else {
    command_result!(true)
  }
}