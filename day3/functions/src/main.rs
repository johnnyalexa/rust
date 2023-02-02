
fn main() {
    println!("Hello, world!");
    another_function(5, 'b');
    expresion();

    let x = five();
    println!("x is {x}");

    let x = plus_one(5);
    println!("x is {x}");
}


fn another_function(value: i32, unit_label: char) {
    println!("The value of x is {value} with label {unit_label}");
}

fn expresion(){
    let y = {
        let x = 3;
        x + 1
    };

    println!("y is {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}