use std::cmp::PartialOrd;
use std::fmt::Display;

// PartialOrd tells compiler that the generic is a type that can be ordered
// Without it, it won't compiule
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Point2<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Traits are like interfaces, provides a template for structs/enums to go off on
trait Summary {
    fn summarize(&self) -> String; // types that implement traits will implement their own method for the trait.

    fn default_summary(&self) -> String {
        // can also define default implementations
        String::from("(Read more)...")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// allows this function to accept all types that implement Summary
fn get_summary(source: &impl Summary) {
    println!("Breaking news! {}", source.summarize())
}

// multiple trait bounds, argument must implement Summary and Display
fn something(item: &(impl Summary + Display)) {}

// Clearer trait bounds with "where" clause
fn some_function<T, U>(t: &T, u: &U) -> i32 where T: Summary + Display, U: Display {
    10
}

// lifetimes
// rule 1: all parameters will be assigned a lifetime parameter that's a reference
// rule 2: if there exists only 1 lifetime parameter, all outputs will be assigned that lifetime
// rule 3: if in the parameters there exists &self, all outputs will be assigned the lifetime of self
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn main() {
    let list_ints = [1, 2, 3, 4, 5, 6];
    let largest_int = largest(&list_ints);

    let list_chars = ['h', 'g', 's', 'z', 'y'];
    let largest_char = largest(&list_chars);

    println!("Largest int: {largest_int}, largest char: {largest_char}");

    let both_integer = Point { x: 5, y: 4 };
    let both_float = Point { x: 5.0, y: 4.1 };
    let both_different = Point { x: 2, y: 2.1 };
}
