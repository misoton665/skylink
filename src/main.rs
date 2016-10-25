extern crate skylink;

use std::env;

use skylink::commands::command;

fn main() {

  let result = command::runCommand(env::args());

  println!("result: {}", result.is_success);
}
