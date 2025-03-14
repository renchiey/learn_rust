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