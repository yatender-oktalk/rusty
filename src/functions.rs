pub fn run() {
  greetings("hello", "jane")
}

fn greetings(greet: &str, name: &str) {
    println!("{} {}", greet, name);
    
    println!("{}", add(5, 6));
    // bid func return value to a variable
    
    let sum = add(5, 16);
    println!("{}", sum);

  // closure
  let n3 = 5;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("{}", add_nums(25, 6));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
