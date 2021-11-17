use std::fs::File;
use std::io::{ErrorKind};

fn main() {
    // Unrecoverable errors
    // panic!("crash and burn!")
    let _v = vec![1, 2, 3];
    // let x = v[99];


    // Recoverable errors
    let file = File::open("hello.txt");
    let _file = match file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(created_file) => {created_file}
                Err(_) => {panic!("Error while creating a file")}
            }
            _other_error => {panic!("problem opening the file")}
        }
    };
    // using unwrap
    let file_2 = File::open("hello.txt").unwrap();
    // using expect
    let file_3 = File::open("world.txt").expect("No such a file in directory");
}
