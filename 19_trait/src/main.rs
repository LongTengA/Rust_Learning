fn main() {
    {
        let vec: [i32; 13] = [1, 2, 3, 45, 6, 8, 324, 24, 324, 23, 423, 4, 2];
        println!("Largest is {}", lagrest(&vec));

        let f_point: Point<i32> = Point { x: 32, y: 23 };

        f_point.print();
    }

    let day1 = NewsArtice {
        headline: "Hello The World! I am LT ,HA HA".to_string(),
        location: "CHINA".to_string(),
        author: String::new(),
        content: "Read te artice I will tell you about the truth".to_string(),
    };

    println!("{}", day1.summarize());
}

/// .
fn lagrest(number: &[i32]) -> i32 {
    let mut lagest = number[0];
    for &i in number {
        if i > lagest {
            lagest = i;
        }
    }
    lagest
}
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn print(&self) {
        println!("Hello World!");
    }
}

struct NewsArtice {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct TweetArtice {
    headline: String,
    location: String,
    reply: bool,
    retweet: bool,
}

trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for TweetArtice {
    fn summarize(&self) -> String {
        format!("{}\n{}", self.headline, self.location)
    }
}

impl Summary for NewsArtice {
    fn summarize(&self) -> String {
        format!("{}\n    {}", self.headline, self.content)
    }
}
