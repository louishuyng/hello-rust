fn main() {
    // Bad
    // let x = 5;
    // println!("The value of x is {}", x);
    // x = 6;
    // println!("The value of x is {}", x);
    
    // Good 
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 100_000;
    
    println!("The value of MAX_POINTS is {}", MAX_POINTS);
}
