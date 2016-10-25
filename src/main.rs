extern crate skylink;

use std::env;

use skylink::commands::command;

fn main() {

  let result = command::run_command(env::args());

  println!("result: {}", result.is_success);
}
