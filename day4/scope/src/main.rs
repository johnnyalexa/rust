fn main() {
    println!("Hello, world!");

    {
        let mut s = String::from("Hello");
        s.push_str(", world!");

        println!("{s}");
    }

    let x = 5;
    let y = x;

    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1={}, s2={}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);
    //println!("s={}", s);

    let x = 5;
    make_copy(x);
    println!("x={}", x);

    let s1 = give_ownership();
    println!("s1={}", s1);

    let s2 = String::from("hello");
    println!("s2={}", s2);

    let s3 = take_and_gives_back(s2);
    println!("s3={},", s3);


    let s4 = String::from("bababim");
    let(s5, len) = calculate_len(s4);
    println!("The len of {} is {}", s5, len);

}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn take_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}