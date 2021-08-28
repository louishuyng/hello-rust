struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn main() {
  let user1 = User {
    username: String::from("louisror"),
    email: String::from("huynguyennbk@gmail.com"),
    sign_in_count: 5,
    active: true
  };
  let mut user2 = build_user(String::from("huynguyennbk@gmail.com"), String::from("louisror"));
  let user3 = User {
    username: String::from("louisror2"),
    email: String::from("huynguyennb2k@gmail.com"),
    ..user1
  };

  user2.username = String::from("rjl18nv");

  println!("username of user1 is {}", user1.username);
  println!("username of user2 is {}", user2.username);
  println!("username of user3 is {}", user3.username);


  struct Color(i32, i32, i32);

  let black = Color(0, 1, 2);
  println!("struct tupple black first color is {}", black.0);
  println!("struct tupple black second color is {}", black.1);
  println!("struct tupple black third color is {}", black.2);
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    sign_in_count: 1,
    active: true
  }
}
