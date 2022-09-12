use std::collections::HashMap;

enum Test {
    Age(i32),
    Name(String),
}
fn main() {
    let mut a: Vec<i32> = Vec::new();

    let mut b = vec![1];

    a.push(1);
    b.push(2);

    let c = &a;
    let d = &mut a;

    let e = vec![Test::Age(1), Test::Name(String::from("小李!"))];
}
