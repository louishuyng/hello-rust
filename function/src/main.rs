fn main() {
    println!("Hello, world!");
    another_function();
    param_function(5, 6);
    expression();
    let y = five();
    println!("The value of y is {}", y);
}

fn another_function() {
    println!("Another Function");
}

fn param_function(x:i32, y:i32) {
    println!("The value of x is {} y is {}", x, y); 
}

fn expression() {
    let _x = 5;
    
    let y = {
       let x = 3;
       x +1
    };
   
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}
