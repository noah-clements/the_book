fn main() {
    print_labeled_measurement(5, 'h');
    expression();
    let x = five();
    println!("The value of five() is: {x}");
    let z = plus_one(5);
    println!("The value of plus_one(5) is: {z}");
}

fn print_labeled_measurement(value: i32, unit: char) {
    println!("The measurement is {value}{unit}");
}

fn expression() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
