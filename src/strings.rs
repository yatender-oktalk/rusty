pub fn run() {
    let mut hello = String::from("Hello ");

    hello.push('w');

    println!("{}", hello);

    hello.push_str(" World!");

    // Capacity
    println!("{}", hello.capacity());

    // Contains
    println!("contains ld? {}", hello.contains("World"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Replace
    let mut new_string = hello.replace("World", "There");
    println!("{}", new_string);

    // Create a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    assert_eq!(2, s.len());
}
