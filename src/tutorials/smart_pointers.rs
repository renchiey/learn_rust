enum List {
    Cons(i32, Box<List>),
    Nil,
}

use self::List::{ Cons, Nil };

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

pub fn main() {
    // box stores data on the heap
    let b = Box::new(5);
    println!("b = {b}");

    // recursive type
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
