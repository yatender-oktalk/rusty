use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1,2,3,4,5];

  println!("{:?}", numbers);

  // Get single value
  println!("Single value: {}", numbers[0]);

  // Reassign value
  numbers[2] = 20;
  println!("{:?}", numbers);

  // Get vector length
  println!("Vector length: {}", numbers.len());

 // Vectors are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice); 

  for n in numbers.iter() {
    println!("{}", n);
  }

  // loop and mutate

  for n in numbers.iter_mut() {
    *n *= 2;
  }

  println!("{:?}", numbers);
}