pub fn run() {
  // This is immutable for two reasons, using the std::str type and as the variable is immutable
  let immutable_string = "Some String";
  // This is immutable for only the reason that the variable is immutable. (Use mut)
  let another_immutable_string = String::from("Hello World");
  // This is mutable
  let mut mutable_string = String::from("Hey there!");

  /* immutable_string.push()/push_str()
   * Error
   * no method named `push` found for reference `&str` in the current scope
   */
  
  /* another_immutable_string.push()/push_str() 
   * Error
   * cannot borrow `another_immutable_string` as mutable, as it is not declared as mutable
   */

  mutable_string.push(',');
  mutable_string.push_str(" and Josh");
  println!("Mutable String: {0}\nImmutable String: {1}\nAnother Immutable String: {2}", mutable_string, immutable_string, another_immutable_string);

  let replaced_string = mutable_string.replace(" and Josh", " I'm Josh");
  println!("{}", replaced_string);
}