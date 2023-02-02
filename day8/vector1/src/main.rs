fn main() {
    println!("Hello, world!");

    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3,4,5,6,7,8,9];

    println!("{:?}, {:?}", v1, v2);

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);

    println!("{:?}", v3);


    let third: &i32 = &v2[2];
    println!("Third element in v1 is {third}");

    let third: Option<&i32> = v2.get(7);
    match third {
        Some(x) => println!("the forth element is {x}"),
        None => println!("There is no element"),
    }

    let first = &v3[0];
    //v3.push(9);
    println!("the first element in v3 is {first}");


    for i in &mut v3 {
        println!("{i}");
        *i += 50;
        println!("{i}");
    }


    let row = vec! [
        SpredasheetCell::Int(3),
        SpredasheetCell::Float(1.2),
        SpredasheetCell::Text(String::from("blue")),
    ];

    for a in &row {
        println!("{:?}", a);
    }
}

#[derive(Debug)]
enum SpredasheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}