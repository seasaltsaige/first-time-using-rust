pub fn run() {
  let age: u8 = 16;
  let check_user_id: bool = false;
  let knows_person: bool = true;

  if age >= 21 || knows_person {
    println!("What would you like to drink?");
  } else if age < 21 && check_user_id {
    println!("Sorry, looks like you're underage.");
  } else {
    println!("I need to see your ID");
  }

  let is_of_age = if age >= 21 { true } else { false };
  println!("{}", is_of_age);
}