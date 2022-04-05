// Reference pointers -> point to a resource in memory

pub fn run() {
  let array1 = [1, 2, 3, 4, 5];
  let array2 = array1;

  println!("{:?}", array1);
  println!("{:?}", array2);

  // with non-primitive, if you assign another variable to a reference, 
  // the original variable will no longer hold the value, you'll need to use the & to point to the resource

  // Vector

  let vec1 = vec![1, 2, 3];
  let vec2 = &vec1;

  println!("{:?}", vec1);
  println!("{:?}", vec2);

}