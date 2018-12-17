use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let file_name = "hello.txt";

    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    // Try to open a file that doesn't exist.
    let file_name = "hello.txt";

    let _f = File::open(file_name).map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("Couldn't create file: {:?}", error);
            })
        } else {
            panic!("There was a problem opening {}: {:?}", file_name, error);
        }
    });

    let username = read_username_from_file();
    match username {
        Ok(username) => println!("{}", username),
        Err(e) => panic!("Error: {:?}", e),
    };
}
