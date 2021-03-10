pub fn run() {
  greeting("Hello there", "John");
  
  // Binding return values to variables
  let get_sum = add(32, 23);
  println!("Sum = {}", get_sum);

  // Closure
  let n3: i32 = 20;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("C Sum: {}", add_nums(3, 6));
}

fn greeting(greet: &str, name: &str) {
  println!("{g} {n}", g = greet, n = name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}