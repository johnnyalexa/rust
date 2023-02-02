use std::fs::File;

fn main() {
    println!("Hello, world!");
    //panic!("crash and burn");

    let v = vec![1,2,3];

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the gile: {:?}", error),
    };
}
