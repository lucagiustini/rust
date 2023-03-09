use std::io;

fn main() {

    println!("Type a number: ");
    let mut x = String::new();

    io::stdin()
            .read_line(&mut x)
            .expect("Not a number");

    let x: i8 = x
        .trim()
        .parse().expect("Not a number");
    
    if x <= 5 {
        println!("x is OK");
    } else {
        println!("x should be > 5");
    }

    let mut y = String::new();
        println!("Type a number: ");
    
    io::stdin()
            .read_line(&mut y)
            .expect("Wrong");

    let y: bool = y
        .trim()
        .parse().expect("");
        println!("OK");

    if y {
        println!("y = TRUE");
    } else {
        println!("y = FALSE");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
