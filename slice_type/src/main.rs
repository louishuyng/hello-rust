fn main() {
  let mut s = String::from("hello world");

  let word = first_world(&s);
  println!("the first word is {}", word);

  s.clear();
  
  // println!("the first word is {}", word); // Can not work since borrow mutatable reference before at line 2
  let my_string_literal = "hello world";

  let word = first_world(&my_string_literal[..]);
  println!("the first word is {}", word);
  
  // let slice = &s[0..2];
  // let len = s.len();
  // let slice1 = &s[..2];
  // let slice2 = &s[3..len];
  // let slice3 = &s[3..];
 
  let a = [1,2,3,4,5];
  let slice = &a[1..3];

  assert_eq!(slice, &[2,3]);
}

fn first_world(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}
