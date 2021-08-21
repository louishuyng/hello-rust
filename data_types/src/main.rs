fn main() {
    let guess: u32 = "42".parse().expect("not a number!");
    println!("the value of guess is {}", guess);

    let float_number: f32 = 3.0;
    println!("the value of float number is {}", float_number);

    let f:bool = false;
    println!("the value of f is {}", f);
    
    let heart_eyed_cat = '😻';
    println!("the cat icon: {}", heart_eyed_cat);

    let tup:(i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the tuple values: {}, {}, {}", x, y, z);

    let a: [i32; 5] = [1,2,3,4,5];
    println!("the array with 5 items type i32: {} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);

    let a = [3;5];
    println!("the array with same value 5 times: {} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);
}
