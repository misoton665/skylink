extern crate skylink;

use std::env;

use skylink::commands::command_runner;

fn main() {

  let result = command_runner::run_command(env::args());

  println!("result: {}", result.is_success);
}
