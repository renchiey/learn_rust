use std::ops::Deref;

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

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // points to the first value of the tuple struct
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

#[derive(Debug)]
enum RefCellList {
    Cons(Rc<RefCell<i32>>, Rc<RefCellList>),
    Nil,
}

use std::rc::{ Rc, Weak };
use std::cell::RefCell;

// Tree DS
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn main() {
    // box stores data on the heap
    let b = Box::new(5);
    println!("b = {b}");

    // recursive type
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Smart pointer
    let y = MyBox::new(6);
    println!("Smart pointer value: {}", *y.deref());

    let c = CustomSmartPointer {
        data: String::from("My Stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("Their Stuff"),
    };

    println!("CustomSmartPointers created");
    drop(d);
    println!("CustomSmartPointer dropped before the end of main");

    // Reference Counter smart pointer
    let q = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));

    println!("Count after creating q = {}", Rc::strong_count(&q));

    // both w and e hold a pointer to q
    let w = RcList::Cons(3, Rc::clone(&q)); // Rc::clone increments the reference count
    let e = RcList::Cons(4, Rc::clone(&q));

    println!("Count after creating w and e = {}", Rc::strong_count(&q));

    {
        let r = RcList::Cons(4, Rc::clone(&q));
        println!("Count after creating r = {}", Rc::strong_count(&q));
    }

    println!("Count after dropping r = {}", Rc::strong_count(&q));

    // RefCell
    let value = Rc::new(RefCell::new(5));

    let i = Rc::new(RefCellList::Cons(Rc::clone(&value), Rc::new(RefCellList::Nil)));
    let o = RefCellList::Cons(Rc::new(RefCell::new(3)), Rc::clone(&i));
    let p = RefCellList::Cons(Rc::new(RefCell::new(4)), Rc::clone(&i));

    *value.borrow_mut() += 10;

    println!("i after = {i:?}");
    println!("o after = {o:?}");
    println!("p after = {p:?}");

    // Tree DS
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("Leaf parent = {:?}", leaf.parent.borrow_mut().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("Leaf parent = {:?}", leaf.parent.borrow_mut().upgrade())
}
