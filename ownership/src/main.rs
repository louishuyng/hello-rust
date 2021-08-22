fn main() {
  let mut s = String::from("hello");
  s.push_str(", world!");

  println!("{}", s);
  
  let s1 = String::from("hello");
  let s2 = s1;

  // Not allow since s1 will invalid when move to s2
  // println!("{}, world!", s1);
  
  // allow since s1 will invalid when move to s2
  println!("{}, world!", s2);
  
  // clone (deep copy) can slove problem
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2);

  let x = 5;
  takes_ownership(s1);

  makes_copy(x);

  let s3 = gives_ownershio();
  println!("{}, world!", s3);

  let s1 = String::from("hello");
  let s3 = takes_and_gives_back(s1);
  println!("{}, world!", s3);

  let (s4, length) = calculate_length(s3);

  println!("The length of '{}' is {}.", s4, length);
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}

fn gives_ownershio() -> String {
  let some_string = String::from("hello");
  
  some_string
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();

  (s, length)
}
