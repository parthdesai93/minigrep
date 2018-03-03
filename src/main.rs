use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args);

  println!("query: {:?}", config.query);
  println!("filename: {:?}", config.filename);

  let mut f = File::open(config.filename).expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading file");

  println!("With text:\n{}", contents);
}

struct Config {
  filename: String,
  query: String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
  }
}
