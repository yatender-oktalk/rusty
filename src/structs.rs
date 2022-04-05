pub struct Person {
  first_name: String,
  last_name: String,
  age: u8,
}

impl Person {
  pub fn new(first_name: &str, last_name: &str, age: u8) -> Person {
    Person {
      first_name: first_name.to_string(),
      last_name: last_name.to_string(),
      age,
    }
  }

  pub fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  pub fn set_last_name(&mut self, last_name: &str) {
    self.last_name = last_name.to_string();
  }

  pub fn set_last_name_mut(&mut self, last_name: &str) {
    self.last_name = last_name.to_string();
  }
}

pub fn run() {
  // struct Color{
  //   red: u8,
  //   green: u8,
  //   blue: u8
  // }

  // let mut c = Color{
  //   red: 255,
  //   green: 0,
  //   blue: 0
  // };

  // println!("Color: {} {} {}", c.red, c.green, c.blue);

  // c.red = 200;
  // println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut person = Person::new("Yatender", "Khurja", 30);
    println!("{}", person.full_name());
    person.set_last_name("Kumar");
    println!("{}", person.full_name());
}