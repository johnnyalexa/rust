use std::io;

fn main() {
    println!("Hello, world!");


    let a = [1, 2, 3, 4, 5];

    println!("please enter an array index:");

    let mut index = String::new();

    loop {
        io::stdin()
            .read_line(&mut index)
            .expect("Failesd to read line");

        let index: usize = match index
            .trim()
            .parse()
            {
                Ok(num) => num,
                Err(_) => continue
            };
        println!("The element {} is {}", index, a[index]);
        break;
    }
}
