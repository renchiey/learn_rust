use std::io;

pub fn variables() {
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    let y = 3;
    
    println!("The value of y is: {y}");
    
    let y = y * 2;

    println!("The value of y is: {y}");

    {
        let y = y * 6;

        println!("The value of y is: {y}");
    }

    println!("Final value of y is: {y}");

}

pub fn data_types() {
    let mut arr: [i32; 5] = [5, 5, 6, 5, 1];

    arr[4] = 6;

    let mut index = String::new();

    println!("Enter an index:");
    io::stdin()
        .read_line(&mut index)
        .expect("hello");

    println!("The index that you entered was: {index}");

    let index: usize = index.trim().parse().expect("SHIT");

    let test_function_out = convert_usize_to_string(index);
    println!("Random function test: {test_function_out}");

    let value = arr[index];
    println!("The value at that index was: {value}");

    println!("Strings and stuff:");

    let mut a = String::from("hello");
    
    a += " Hello";

    println!("String: {a}");

   
}

fn convert_usize_to_string(x: usize) -> String {
    let out: String = {
        let y = x + 2;
        y.to_string()
    };

    out
}

pub fn loops() {
    let mut count = 0;

    'outer_loop: loop {
        println!("count: {count}");

        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");

            if remaining == 7 {
                break;
            }
            if count == 2  {
                break 'outer_loop;
            }
            
            remaining -= 1;

        }
        count += 1;
    }

    

    println!("ARRAY LOOP");

    let arr= ["hi", "bye", "guy", "die", "fly"];

    let mut i = 1;
    for s in arr {
        println!("Word {i} is {s}");
        i += 1;
    }
    println!("END OF ARRAY LOOP")


}

