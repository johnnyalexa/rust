fn main() {
    println!("Hello, world!");

    let mut s1 = String::new();

    let data = "intiial text";
    let s2 = data.to_string();
    let s3 = "initial text".to_string();


    let mut s4 = String::from("foo ");
    let s5 = "bar";
    s4.push_str(s5);
    println!("{s4}");
    println!("{s5}");
    s4.push('!');
    println!("{s4}");

    let s6 = s4 + &s5;
    println!("{s6}");
    //println!("{s4}");
    println!("{s5}");

    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

    let s10 = format!("{s7}-{s8}-{s9}");
    println!("{s10} , {s9}");


    let h = &s7[0..1];
    println!("{h}");

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    for c in hello.chars() {
        println!("{c}");
    }
}
