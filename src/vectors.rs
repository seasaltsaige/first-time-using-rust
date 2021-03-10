pub fn run() {
  let mut vector: Vec<&str> = vec!["Hello", "World"];
  vector.push("!");

  println!("Updated Vector: {:?}", vector);
  println!("{hello}", hello=vector.join(" "));

  vector.pop();
  println!("{:?}", vector);

  for val in vector.iter() {
    println!("Value: {}", val);
  }

  for val in vector.iter_mut() {
    *val = "Bruh";
  }

  println!("{:?}", vector)
}