pub type CommandArgs<'a> = &'a [String];

pub enum Command<'a> {
  FindCommand(CommandArgs<'a>),
  AddCommand(CommandArgs<'a>),
  NoCommand,
}

pub struct CommandResult {
  pub is_success: bool,
}

impl CommandResult {
  pub fn new(is_success: bool) -> CommandResult {
    CommandResult{ is_success: is_success }
  }
}

fn select_command<'a>(name: &str, args: &'a [String]) -> Command<'a> {
  match name {
    "find" => Command::FindCommand(args),
    "add" => Command::AddCommand(args),
    _ => Command::NoCommand,
  }
}

macro_rules! command_result {
  ($v:expr) => (CommandResult::new($v));
}
