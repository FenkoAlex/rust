use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

use edu::{notify, NewsArticle, Summary, Tweet};

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
    println!("p.x = {}", p.x);
    let p = Point { x: 5.0, y: 10_f32 };
    p.distance_from_origin();

    let article = NewsArticle {
        headline: "Test".to_string(),
        location: "String".to_string(),
        author: "String".to_string(),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("{}", article.summarize());

    notify(&article);

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let a = String::from("value");
    let b = String::from("longer_value");

    println!("{}", longest_with_an_announcement(&a, &b, "announcement"));
}
