use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  let command = args[1].clone();
  let name = "Max";

  println!("Command: {}", command);

  if command.to_lowercase() == "hey" {
    println!("Hi {}, how are you?", name)
  } else if command.to_lowercase() == "bye" {
    println!("Bye {}", name);
  }
}