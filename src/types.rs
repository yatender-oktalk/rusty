/*
Primitive Types

Integers: u8, u16, 
Float: f32, f64
Boolean
Characters (char)
Tuples
Arrays

*/

// Statically typed lang, it must know all types at compile time
// compiler can infer based on usage (sometimes rarely it can be wrong)

pub fn run() {
  // Default is "i32"
  let x = 1;

  // Default is "f64"
  let y = 2.5;

  // Add explicit type
  let z: i64 = 2147483648;

  let is_active: bool = true;

  let is_greater: bool = 10 > 5;

  let a1 = 'a';
  let face = '\u{1F600}';
  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
