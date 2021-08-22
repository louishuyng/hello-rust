fn main() {
  let mut s1 =  String::from("hello") ;

  let len = calculate_length(&s1);
  println!("len is {}", len);

  change(&mut s1);
  println!("s1 is {}", s1);

  // Can only have one mutable reference
  // let r1 = &mut s1;
  // let r2 = &mut s1;
  // println!("{}, {}", r1, r2);
  

  let mut s = String::from("hello");

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  println!("{}, {}", r1, r2);

  let r3 = &mut s; // can use after r1 and r2 are no longer used
  println!("{}", r3);

  // let reference_to_nothing = dangle();
  let reference = not_dangle();
  println!("{}", reference);
}

fn calculate_length(s: &String) -> usize {
  // Can not be borrowed as mutable
  // s.push_str(", world!");
  s.len()
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

// fn dangle() -> &String {
//   let s = String::from("hello");
// 
//   &s
// }

fn not_dangle() -> String {
  let s = String::from("hello");

  s
}
