// Vars are block scoped
// immutable by default
// holds the primitive data or references to data

pub fn run() {
  let name = "Brad";
  let mut age = 37;
  println!("My name is {} and I am {}", name, age);
  age = 56;
   println!("My name is {} and I am {}", name, age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Brad", 37);
  println!("{} is {}", my_name, my_age);
}