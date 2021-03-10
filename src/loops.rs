pub fn run() {
  let mut count = 0;
  // Infinite loop
  loop {
    count += 1;
    println!("Number: {}", count);
    if count == 20 {
      break;
    }
  }

  let mut counter = 0;

  // While loop
  while counter <= 100 {
    if counter % 15 == 0 {
      println!("FizzBuzz");
    } else if counter % 3 == 0 {
      println!("Fizz");
    } else if counter % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", counter);
    }
    counter += 1;
  }

  // For range loop
  for x in 0..100 {
    println!("{}", x + 1);
  }

}