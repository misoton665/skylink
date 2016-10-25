use std::env::Args;

use common::*;
use commands::find_command::*;
use commands::command_common::*;

fn runHelp() -> CommandResult {
  println!("This is Help.");
  CommandResult::new(true)
}

fn selectCommand<'a>(name: &str, args: &'a [String]) -> Command<'a> {
  match name {
    "find" => Command::FindCommand(args),
    _ => Command::NoCommand,
  }
}

pub struct CommandRunner;

impl CommandRunner {

  fn run(command: &Command) -> CommandResult {
    match *command {
      Command::FindCommand(args) => runFindCommand(args),
      Command::NoCommand => runHelp(),
    }
  }
}

pub fn runCommand(commandline_args: Args) -> CommandResult {
  if commandline_args.len() == 1 {
    return CommandResult::new(false)
  }

  let args_vec = commandline_args.skip(1).collect::<Vec<String>>();
  let (name, args) = args_vec.split_at(1);
  let command = selectCommand(&name[0], &args);

  CommandRunner::run(&command)
}
