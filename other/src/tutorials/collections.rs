use std::collections::HashMap;

pub fn main() {
    //vectors();

    //strings();

    hashmaps();
}

fn vectors() {
    let mut v: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3]; // vector instantiation macro

    v.push(5);
    v.push(6);
    v.push(10);

    let indexing = &v[1];
    let getting = v.get(1);

    match getting {
        Some(val) => println!("The element in index 1 using .get: {val}"),
        _ => println!("The element does not exist"),
    }

    println!("The element in index 1 is: {indexing}");

    // borrowing with vectors

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // borrows the first value

    v.push(6);

    println!("The first element is: {}", &v[0]); // can't use "first" variable

    // iterating
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 10; // *i dereferences the pointer to the i32 allowing it to be updated
    }

    println!("New values:");
    for i in &v {
        println!("{i}");
    }
}

fn strings() {
    // strings are implemented as a vector of bytes
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2; // &s2 because of add implementation = add(self, s:&str)

    println!("Concatenated strings: {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("Formatted string: {s}");

    // best way to iterate through strings, have to specify characters or bytes
    for c in s.chars() {
        println!("{c}");
    }

    for b in s.bytes() {
        println!("{b}");
    }
}

fn hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let shit = &String::from("Blue");

    let score = scores.get("Blue").copied().unwrap_or(0);

    println!("{score}");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
}
