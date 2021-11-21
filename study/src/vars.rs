// Variables hold primitive data or references to data
// Variables are immutiable by default
// Rust is a block-scoped language
pub fn run() {
  let name = "Brad";

  let mut age = 26;
  println!("My Name is {} and I am {}", name, age);
  age = 27; 
  println!("My Name is {} and I am {}", name, age);

  // Define constant
  const ID: i32 = 001;

  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Brad", 26);
  println!("My Name is {} and I am {}", my_name, my_age);
}