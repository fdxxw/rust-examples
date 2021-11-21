use std::mem;
pub fn run() {
  // let numbers: [i32; 5] = [1,2,3,4,5];
  let mut numbers: Vec<i32> = vec![1,2,3,4,5];

  // Re-assign value
  numbers[2] = 20;

  // Add on to vector
  numbers.push(6);
  numbers.push(7);
  numbers.push(8);

  // Pop of last value
  numbers.pop();
  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);


  // Get Vector Length
  println!("Vector Length: {}", numbers.len());

  // Vector are stack allocated
  println!("Vector occupies: {} bytes", mem::size_of_val(&numbers));

  // Get slice

  let slice: &[i32] = &numbers[1..3];

  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop & mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers Vec: {:?}", numbers);
}