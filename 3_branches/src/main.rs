use std::thread::sleep_ms;

fn main() {
    let number = 3;
    if number > 3 {
        println!("number is greater than 3");
    } else {
        println!("number is less than 3");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {}", number);

    loop_();

    while_loop();
}

fn loop_() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop(){
    let mut i = 0;
    while i < 10 {
        print!("{}",i);
        sleep_ms(1000);
        i += 1;
    }
}
