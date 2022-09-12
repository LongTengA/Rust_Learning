use std::collections::HashMap;
fn main() {
    //可显式指定也可不显式指定（前提是可以推断）
    let mut map = HashMap::new();
    map.insert(1, String::from("long"));
    map.insert(2, String::from("long"));
    //collect();

    map.entry(1).or_insert("default".to_string());
    map.entry(23).or_insert("Hello".to_string());

    let e = map.entry(10);
    println!("{:?}", e);
    for (k, v) in map {
        //println!("key = {:<10}| value = {:<10}", k, v);
    }

    let mut map2 = HashMap::new();
    let str = "Hello World! Hello @orlda L L L SE SD SS";
    for i in str.split_whitespace() {
        let index = map2.entry(i).or_insert(0);
        *index += 1;
    }
    for (k, v) in map2 {
        println!("key = {:<10}| value = {:<10}", k, v);
    }
}

fn collect() {
    let key = [
        String::from("LONGTENG"),
        String::from("TEgagaST"),
        String::from("TESs2T"),
        String::from("TEsST"),
        String::from("TEfasST"),
        String::from("TEasdST"),
        String::from("TEasfST"),
    ];
    let value = [1, 23, 23, 4, 5, 6, 7];

    let test: HashMap<_, _> = key.iter().zip(value.iter()).collect();

    for (k, v) in test {
        println!("key = {:<10}| value = {:<10}", k, v);
    }
}
