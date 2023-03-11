fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    named_loop();
    liftoff();
    for_loop();
    count_down();
}

fn count_down() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("Liftoff!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn named_loop() {
    let mut counter = 0;
    'counting_up: loop {
        println!("counting up: {}", counter);
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End count: {counter}");
}

fn liftoff() {
    let mut countdown = 10;
    while countdown > 0 {
        println!("{countdown}!");
        countdown -= 1;
    }
    println!("Liftoff!");
}   
