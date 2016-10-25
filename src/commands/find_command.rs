use commands::command_common::*;

pub fn run_find_command(args: CommandArgs) -> CommandResult {
  println!("find command");
  if args.len() == 0 {
    println!("find command was called, but argument is nothing!!!!!");
    CommandResult::new(false)
  } else {
    CommandResult::new(true)
  }
}