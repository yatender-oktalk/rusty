// group together different value types
// max 12 elems

pub fn run() {
  let person: (&str, &str, i8) = ("Yatender", "Khurja", 30);
  println!("{} is from {} and is {}", person.0, person.1, person.2);
}
