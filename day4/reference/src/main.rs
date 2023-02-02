fn main() {
    println!("Hello, world!");

    let s1 = String::from("cicaciao");
    let len = calculate_len(&s1);

    println!("The len of {} is {}", s1, len);

    let mut s2 = String::from("Hello");
    change(&mut s2);
    println!("s2 became {}", s2);

    let mut s3 = String::from("Textbook");
    let r1 = &s3;
    
    println!("r1 = {}", r1);
    let r2 = &mut s3;
    println!("r2 = {}", r2);

    let ref_to_nothing = dangle();
    println!("some ref = {}", ref_to_nothing);
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("Dangling");
    s
}