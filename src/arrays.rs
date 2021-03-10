use std::mem;
pub fn run() {

  let mut some_numbers: [i32; 14] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
  
  println!("{:?}", some_numbers);

  some_numbers[0] = 23;
  println!("{:?}", some_numbers);
 
  println!("Arr len: {}", some_numbers.len());

  println!("Arr stack mem: {} Bytes", mem::size_of_val(&some_numbers));

  let slice_of_some_numbers: &[i32] = &some_numbers[0..2];
  println!("{:?}", slice_of_some_numbers);
}