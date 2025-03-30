use macros::test;

fn main() {
    println!("Hello, world!");

    // Declarative macro
    let lst = test![{(1, fart), (2, fart), (3, fart)}];

    let res = custom_macro_derive::add(1, 2);

    println!("res: {res}");
    println!("test result: {lst:?}");
}
