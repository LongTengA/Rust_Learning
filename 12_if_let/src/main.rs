fn main() {
    let v = Some(2);
    match v {
        Some(8) => println!("100"),
        _ => (),
    }

    if let Some(8) = v {
        println!("100");
    } else {
        println!("others")
    }
}
