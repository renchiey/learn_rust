use std::fs::File;
use std::io::{ self, ErrorKind, Read };

pub fn main() {
    // panic!("crash and burn"); - unrecoverable error (When the program must terminate)

    // Recoverable errors have return type Result<T, E>
    // Two possible values: Ok(T) | Err(E)
    let file_result = File::open("hello.txt");

    let file = match file_result {
        Ok(f) => f,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    // Matching different errors

    let file2_result = File::open("bye.txt");

    let file2 = match file2_result {
        Ok(f) => f,
        Err(error) =>
            match error.kind() {
                ErrorKind::NotFound =>
                    match File::create("hello.txt") {
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem with creating file: {e:?}"),
                    }
                other_error => { panic!("Problem opening the file: {other_error:?}") }
            }
    };

    // unwrap will return the value inside an Ok or call panic! if there is an error
    let file3 = File::open("goodbye.txt").unwrap();

    // expect lets you log a message with the panic! macro
    let file4 = File::open("gday.txt").expect("gday.txt should be included in this project");
}

fn propogating_error() -> Result<String, io::Error> {
    let mut username_file = File::open("file.txt")?; // will return Error if file cannot be opened

    let mut username = String::new();

    username_file.read_to_string(&mut username)?; // will also return error if cannot read to string
    Ok(username)

    // more concise method: chaining
    // let mut username = String::new();

    // File::open("hello.txt")?.read_to_string(&mut username)?;

    // Ok(username)
}
