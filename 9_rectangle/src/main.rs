//计算一个正方形的面积
#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}
//方法语法
impl Rectangle {
    fn area(&self) -> u32 {
        self.h * self.w
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }

    fn square(len: u32) -> Rectangle {
        Rectangle { w: len, h: len }
    }
}

fn main() {
    let rec1 = Rectangle { w: 10, h: 32 };
    let rec2 = Rectangle { w: 123, h: 321 };
    let rec3 = Rectangle::square(12);
    println!("The area is {}", rec1.area());
    println!("Rectangle info {:#?}", rec1);
    if rec1.can_hold(&rec2) {
        println!("rec1 can hold rec2");
    } else if rec1.can_hold(&rec3) {
        println!("rec1 can hold rec2");
    } else if rec2.can_hold(&rec3) {
        println!("println!(\"rec1 can hold rec2\");")
    }
}
#[allow(dead_code)]
fn area(rec: &Rectangle) -> u32 {
    rec.w * rec.h
}
