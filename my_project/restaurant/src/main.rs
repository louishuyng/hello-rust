use std::fmt::Result;
use std::io::Result as IoResult;

fn main() {
  restaurant::eat_at_restaurant();
  restaurant::hosting::add_to_waitlist();
}

// fn function1() -> Result {
//   // -snip
// }
// 
// fn function2() -> IoResult<()> {
//   // -snip
// }
