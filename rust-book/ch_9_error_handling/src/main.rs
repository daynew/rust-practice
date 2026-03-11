use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => panic!("Problem opening the file: {error:?}"),
        },
    };

    let username =
        read_username_from_file().expect("Something when wrong trying to load hello.txt");
    println!("username={username}");
    let username =
        read_username_from_file_v2().expect("Something when wrong trying to load hello.txt");
    println!("username={username}");
    let username =
        read_username_from_file_v3().expect("Something when wrong trying to load hello.txt");
    println!("username={username}");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username = String::new();
    // Note the '?'
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
