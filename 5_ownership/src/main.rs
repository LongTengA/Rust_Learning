fn main() {
    let s1 = String::from("Hello");

    let s2 = s1.clone();

    //if not use clone,it`s error
    println!("{}", s1);

    println!("{}", s2);

    take_ownership(s1);
    //error s1 was droped
    // println!("{}", s1);

    let s1 = give_ownership();
    println!("{}", s1);

    let s1 = take_and_give_ownership(s1);

    println!("{}", s1);
}

fn take_ownership(str: String) {
    println!("{} is drop!", str);
}

fn give_ownership() -> String {
    let some_str = String::from("World!");
    some_str
}

fn take_and_give_ownership(str: String) -> String {
    str
}
