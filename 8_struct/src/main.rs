struct User {
    name: String,
    age: i32,
    active: bool,
    email: String,
}
fn main() {
    let user1 = User {
        name: String::from("龙腾"),
        age: 32,
        active: true,
        email: String::from("1557068259@qq.com"),
    };

    let user2 = build_user("long", 32);
    print_user(&user2);

    print_user(&user1);
    let user3 = User {
        email: String::from("Hello"),
        ..user1 //结构体更新语法
    };
    print_user(&user3);

    //元组结构体
    struct Point(f64, f64, f64);
    let x = Point(0.0, 0.0, 0.0);
}
//因为示例 5-4 中的参数名与字段名都完全相同，我们可以使用 字段初始化简写语法（field init shorthand）来重写 build_user，
//这样其行为与之前完全相同，不过无需重复 email 和 username 了，如示例 5-5 所示。
fn build_user(name: &str, age: i32) -> User {
    User {
        name: (String::from(name)),
        age: (age),
        active: (true),
        email: (String::from("None")),
    }
}

fn print_user(user: &User) {
    println!(
        "User info\nactive = {}\nage = {}\nemail = {}\nname = {}",
        &user.active, &user.age, &user.email, &user.name
    );
}
