// Arrays - Fixed list where elements are the same data types.
use std::mem;
pub fn run() {
  // let numbers: [i32; 5] = [1,2,3,4,5];
  let mut numbers: [i32; 5] = [1,2,3,4,5];

  // Re-assign value
  numbers[2] = 20;
  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);


  // Get Array Length
  println!("Array Length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array occupies: {} bytes", mem::size_of_val(&numbers));

  // Get slice

  let slice: &[i32] = &numbers[1..3];

  println!("Slice: {:?}", slice);
}