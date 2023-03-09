fn main() {

    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 605;
    println!("The value of y is: {THREE_HOURS_IN_SECONDS}");

    const FOUR_HOURS_IN_SECONDS: u32 = 50;
    println!("The value of z is: {FOUR_HOURS_IN_SECONDS}");

    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of x in the inner scope is: {z}");
    }

    println!("The value of x is: {z}");

    let _spaces = "     "; //string
    println!("The value of space is: {_spaces}");
    let _spaces = _spaces.len(); //number
    println!("The value of space is: {_spaces}");
}
