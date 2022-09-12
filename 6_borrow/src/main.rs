fn main() {
    let str = String::from("Hello World!");
    test_borrow(&str);
    println!("len = {}", test_borrow(&str));

    let mut str2 = String::from("Hello World!");
    let len = change(&mut str2);
    println!("{} len = {}", str2, len);
    //多个可变引用

    let mut s = 13;

    let a = &mut s;

    let b = &mut s;
}

fn test_borrow(str: &String) -> usize {
    println!("{}", str);
    //str.push_str("Hello ");
    str.len()
}

fn change(str: &mut String) -> usize {
    str.push_str("Rust!");
    str.len()
}
