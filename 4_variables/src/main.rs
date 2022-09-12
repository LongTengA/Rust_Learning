fn main() {
    const PI: f64 = 3.145926;
    println!("{}", PI);

    let a = 5;
    println!("{a}");
    let a = 10;
    println!("{a}");

    let space = "      ";
    let space = space.len();
    println!("str len = {}", space);

    let tup: (i32, u32, f64) = (32, 32, 31.023);
    println!("tup = {} {} {}", tup.0*3, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("tup x={} y={} z={}", x, y, z);

    let tupx = tup.0;
    let tupy = tup.1;
    let tupz = tup.2;
    println!("tup x={} y={} z={}", tupx, tupy, tupz);
    //array
    let a = [1, 3, 4, 5];
    for i in a {
        print!("{} ", i);
    }

    let a = [3; 5];
    for i in a {
        print!("{} ", i);
    }
    println!();
    my_function(3, 2);

    let x = return_fun();
    println!("return number is = {}", x);
}

fn my_function(x: i32, y: i32) {
    println!("Hello My function!");
    println!("x = {},y = {}", x, y);
}

fn return_fun() -> i32 {
    1000
}
