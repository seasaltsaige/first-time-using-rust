extern crate regex;
use regex::Regex;

pub fn run() {
  let regex = Regex::new(r"\d+").unwrap();
  println!("{}", regex.is_match("23214"));
  // Regex is very weird
}