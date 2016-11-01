extern crate skylink;

use std::env;

use skylink::commands::command_runner;
use skylink::domain::sky_environment::*;
use skylink::domain::link::*;

fn main() {

  let result = command_runner::run_command(env::args());

  println!("result: {}", result.is_success);

  let mut sky_env = SkyEnvironment::empty_env();

  let link1 = Link::new("hoge", "maguro", false);
  let link2 = Link::new("out", "out", true);

  sky_env.add_link(link1);

  println!("link?: {}", &sky_env.find_link("hoge").unwrap_or(&link2).id);

  sky_env.delete_link("hoge");

  println!("link?: {}", &sky_env.find_link("hoge").unwrap_or(&link2).id);

}
