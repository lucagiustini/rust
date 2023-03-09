fn main() {
    println!("Hello");

    another_function();
    operation();
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("The value of x is : {x}");
    let x = plus_one(5);
    println!("The value of x is : {x}");
}

fn another_function() {
    println!("another_function");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measures are: {value} and {unit_label}");
}

fn operation() {
    let z = {
        let x = 6;
        x + 1
    };
    let y = z - 6;
    println!("{y} = y, {z} = z")
}

fn five() -> i8 {
    127
}

fn plus_one(x: i8) -> i8 {
    x + 1
}