#[derive(Debug)]
enum IpAddrkind {
    V4(u8, u8, u8 ,u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    println!("Hello, world!");
    let four = IpAddrkind::V4(127,0,0,1);
    let six  = IpAddrkind::V6(String::from("::1"));

    dbg!(four);
    dbg!(six);


    let msg1 = Message::Quit;
    dbg!(&msg1);
    msg1.call();
    let msg2 = Message::Write(String::from("test"));
    dbg!(&msg2);
    msg2.call();

    let x: i8 =5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
