fn main() {
    let str1 = "你好世界";
    for i in str1.as_bytes(){
        println!("{:x}",i);
    }
    println!("{}",&str1[0..6])
}
