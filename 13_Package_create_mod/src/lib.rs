mod my_lib {
    pub mod test {
        pub fn add_to_waitlist() {}
    }

    pub struct Season {
        pub summer: i32,
        spring: i32,
    }
    impl Season {
        pub fn construct(x: i32, y: i32) -> Season {
            Season {
                summer: x,
                spring: y,
            }
        }
    }
}

fn test() {
    my_lib::test::add_to_waitlist();

    let day = my_lib::Season::construct(1, 2);
    println!("{}{}",day.summer,2);
}
