extern crate skylink;

use std::env;

use skylink::link::Link;
use skylink::commands::command;

fn main() {
  let l = Link::new("pom", "hom", false);
  let k = Link::new("pom", "hoge", false);

  println!("l: {}", &l.id);

  println!("k: {}", &k.id);

  println!("l == k: {}", l == k);

  let list = vec![1,2,3,4,5];

  let p = list.iter().fold(Vec::new(), |mut acc, x| {acc.push(x); acc});

  println!("p: {}", &p[0]);

  let result = command::runCommand(env::args());

  println!("result: {}", result.is_success);
}
