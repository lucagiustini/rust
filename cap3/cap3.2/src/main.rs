use std::io;

fn main() {

    //  INTEGER
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    let number_i: i8 = -128;
    //let number_i: i8 = 128; -128 to 127
    println!("{number_i}");
    let number_u: u8 = 254; //u8 = 0 to 255
    //let number_u: i8 = 255;
    println!("{number_u}");

    //  FLOAT
    let x = 2.0; // f64
    println!("{x}");
    let y: f32 = 3.0; // f32
    println!("{y}");
    
    // addition
    let sum = 5 + 10;
    println!("{sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("{difference}");
    // multiplication
    let product = 4 * 30;
    println!("{product}");
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{quotient}");
    println!("{truncated}");
    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");

    //  BOOLEAN
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("{t},{f}");

    //  CHARACTER
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c},{z},{heart_eyed_cat}");

    //  TUMPLE
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{x},{y},{z}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred},{six_point_four},{one}");

    //  ARRAY
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{}, {}", first, second);
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];
    println!("{}, {}, {}", months[5], b[1], c[1]);

    let arr = [1, 2, 3, 4, 5];
    println!("Array contents: {:?}", arr);
    for elem in arr {
        println!("{}", elem)
    }

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    
}
