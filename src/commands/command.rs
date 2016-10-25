use std::env::Args;

use commands::find_command::*;
use commands::add_command::*;
use commands::command_common::*;

fn run_help() -> CommandResult {
  println!("This is Help.");
  CommandResult::new(true)
}

struct CommandRunner;

impl CommandRunner {

  fn run(command: &Command) -> CommandResult {
    match *command {
      Command::FindCommand(args) => run_find_command(args),
      Command::AddCommand(args) => run_add_command(args),
      Command::NoCommand => run_help(),
    }
  }
}

pub fn run_command(commandline_args: Args) -> CommandResult {
  // コマンドライン引数の1つ目はファイル名。
  // なのでコマンドに実行には最低で2つ以上必要。
  if commandline_args.len() <= 1 {
    let no_command = Command::NoCommand;
    return CommandRunner::run(&no_command)
  }

  let args_vec = commandline_args.skip(1).collect::<Vec<String>>();
  let (name, args) = args_vec.split_at(1);
  let command = select_command(&name[0], &args);

  CommandRunner::run(&command)
}
