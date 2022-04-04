pub fn run() {
  // printing
  println!("Hello from print.rs");

  // basic formatting
  println!("{} {}", 1, 5);

  // positional args
  println!("{0} {1} {0}", 1, 5);

  // named args
  println!("{name} {salary} {name}", name = "ram", salary = 5);

  // placeholder traits
  println!("Binary: {0:b} Octal: {1:o} Decimal: {2:} Hex: {2:x}", 10, 20, 50);

  // placeholder for debug trait
  println!("{:?}", (12, true, "hello"));
}
