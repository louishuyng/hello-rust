struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size
    }
  }
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
  
  let rect1 = Rectangle {
      width: 30,
      height: 50,
  };
  
  let rect2 = Rectangle {
      width: 10,
      height: 40,
  };
  
  let rect3 = Rectangle {
      width: 60,
      height: 45,
  };

  println!("The area of the rectangle is {} square pixels.", area(&rect1));
  println!("The area of the rectangle is {} square pixels.", rect1.area());

  println!("rect1 is {:?}", rect1);
  println!("rect1 is {:#?}", rect1);
  
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
  
  println!("Square {:#?}", Rectangle::square(10));
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    sign_in_count: 1,
    active: true
  }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
