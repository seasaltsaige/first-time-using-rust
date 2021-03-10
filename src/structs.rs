// Traditional Struct
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// Tuple Struct
struct SecondColor(u8, u8, u8);

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  fn new(f_name: &str, l_name: &str) -> Person {
    Person {
      first_name: f_name.to_string(),
      last_name: l_name.to_string(),
    }
  }
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }
  fn set_last_name(&mut self, new_last_name: &str) -> () {
    self.last_name = new_last_name.to_string();
  }
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() -> () {
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };

  c.green = 23;

  println!("Color: {} {} {}", c.red, c.blue, c.green);

  let mut color = SecondColor(255, 6, 2);

  color.1 = 34;

  println!("Color: {} {} {}", color.0, color.1, color.2);


  let mut person = Person::new("John", "Smith");
  person.set_last_name("Doe");

  println!("Person Full Name: {}", person.full_name());
  println!("Person: {} {}", person.first_name, person.last_name);
  println!("Tupple Name: {:?}", person.to_tuple());
}