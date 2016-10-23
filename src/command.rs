use std::env::Args;

use common::SStr;

type CommandArgs = &'static [String];

fn runFindCommand(args: CommandArgs) -> CommandResult {
  if args.len() == 0 {
    CommandResult::new(false)
  } else {
    CommandResult::new(true)
  }
}

fn runHelp() -> CommandResult {
  println!("This is Help.");
  CommandResult::new(true)
}

fn selectCommand(name: &str, args: &[String]) -> Command {
  Command::NoCommand
}

pub enum Command {
  FindCommand(CommandArgs),
  NoCommand,
}

pub struct CommandResult {
  pub is_success: bool,
}

impl CommandResult {
  fn new(is_success: bool) -> CommandResult {
    CommandResult{ is_success: is_success }
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

  println!("name: {}", &args.len());

  CommandRunner::run(&command)
}
