//use std::io;

fn main() {
    //let mut day: String = String::new();
    //io::stdin().read_line(&mut day).expect("error to readline");

    //println!("{}", day);

    let mut x: String = String::new();
    plus_five(&mut x);
    println!("{}", x);

    let mut y = 10;
    plus_five2(&mut y);
    println!("{}", y);

    let mut test = Coin::Dime;
    test = Coin::Rmb(China::Sicau);
    println!("{}", value_in_coin(&test));
}

fn plus_five(x: &mut String) {
    x.insert_str(0, "5");
}
fn plus_five2(x: &mut i32) {
    *x += 5;
}

enum Coin {
    Rmb(China),
    Penny,
    Dime,
    Quarter,
}
#[derive(Debug)]
enum China {
    Sicau,
    Peking,
    Scu,
}
fn value_in_coin(coin: &Coin) -> i32 {
    match coin {
        Coin::Dime => 1,
        Coin::Penny => 2,
        Coin::Quarter => 3,
        Coin::Rmb(provnec) => {
            println!("China from {:?}", provnec);
            4
        }
    }
}
