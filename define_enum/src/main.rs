enum IpAddrKind {
  V4,
  V6,
}

enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String)
}

#[derive(Debug)]
enum Message {
  Quit,  // has no data associated with it at all.
  Move { x: i32, y: i32 }, // includes an anonymous struct inside it.
  Write(String), // includes a single String.
  ChangeColor(i32, i32, i32), // includes three i32 values.
}

impl Message {
  fn call(&self) {
    println!("{:?}", self);
  }
}

fn main() {
  let x = 5;
  let home = IpAddr::V4(127, 0, 0, 1);
  let loopback = IpAddr::V6(String::from("::1"));

  let m = Message::Write(String::from("hello"));
  m.call();

  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;
}
