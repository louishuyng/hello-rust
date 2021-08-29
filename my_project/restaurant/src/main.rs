use std::fmt::Result;
use std::io::Result as IoResult;
use rand::Rng;

fn main() {
  restaurant::eat_at_restaurant();
  restaurant::hosting::add_to_waitlist();
  
  let secret_number = rand::thread_rng().gen_range(1..101);
  println!("secret number is {}", secret_number);
}

// fn function1() -> Result {
//   // -snip
// }
// 
// fn function2() -> IoResult<()> {
//   // -snip
// }
