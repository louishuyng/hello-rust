#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
  Alabama,
  Alaska, 
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),   
}

fn main() {
  let penny = value_in_cents(Coin::Penny);
  let quarter = value_in_cents(Coin::Quarter(UsState::Alabama));
  
  println!("value is {}", penny);
  println!("value is {}", quarter);

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  let some_u8_value = 2;

  match some_u8_value {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => (),
  }

  let mut count = 0;
  
  let coin = Coin::Quarter(UsState::Alabama);
  if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
  } else {
    count += 1;
  }

  println!("count is {}", count)
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky Penny");
      0b1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    },
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}
