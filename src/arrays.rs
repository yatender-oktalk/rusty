use std::mem;

pub fn run() {
  let mut numbers: [i32; 5] = [1,2,3,4,5];

  println!("{:?}", numbers);

  // Get single value
  println!("Single value: {}", numbers[0]);

  // Reassign value
  numbers[2] = 20;
  println!("{:?}", numbers);

  // Get array length
  println!("Array length: {}", numbers.len());

 // Arrays are stack allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice); 
}