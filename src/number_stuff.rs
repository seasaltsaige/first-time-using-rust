pub fn run() {
  let max_32_int = std::i32::MAX;
  println!("Max 32 int: {}", max_32_int);

  let x: i8 = 33;
  let y: i8 = 23;

  // Type casting
  let z = (x + y) as u128;
  println!("Cast: {}", z);
}