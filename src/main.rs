extern crate skylink;

use std::env;

use skylink::commands::command_runner;
use skylink::domain::sky_environment::*;
use skylink::domain::link::*;

fn main() {

  let result = command_runner::run_command(env::args());

  println!("result: {}", result.is_success);
}
