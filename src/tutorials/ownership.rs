pub fn ownership() {

    let arr = ["hi", "bye", "die", "fly"];

    let res = parse_array_as_string(&arr);

    println!("Parsed array: {res}");

    let words = String::from("hello world");

    let res = first_word(&words);

    println!("First word: {res}");

}

fn parse_array_as_string(arr: &[&str]) -> String {
    let mut output = String::new();
    output += "[";

    let mut i: usize= 0;

    while i < arr.len() {
        output += arr[i];
        if i < arr.len() - 1 {
            output += ", "
        }

        i += 1;
    }

    output += "]";

    return output;
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}