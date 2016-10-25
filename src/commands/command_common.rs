pub type CommandArgs<'a> = &'a [String];

pub enum Command<'a> {
  FindCommand(CommandArgs<'a>),
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
