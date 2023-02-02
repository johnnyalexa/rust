

fn main() {
    println!("Hello, world!");
    loop_temp();
    let mut i = 1;
    while i < 10 {
        println!("Fibonaci number {} is {}", i, generate_nth_fibonaci(i));
        i += 1;
    }
}


// convert temperatuure betweeb Fahrenheit and Celsius

fn convert_f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn loop_temp() {
    let input_tem = [ -20.0, -10.0, 0.0, 10.0, 20.0, 30.0, 40.0];

    for t in input_tem {
        println!("{} Fahreinheit is {} celsius", t, convert_f_to_c(t));
    }
}

fn generate_nth_fibonaci(n: i32) -> i32 {

    let start = (0, 1);

    let mut fib = match n {
        1 => 0,
        2 => 1,
        _ => 0
    };
    //let mut fib = 0;
    let (mut a, mut b) = start;
    //let mut b = 1;
    let mut count = 2;
    while count < n {
        fib = a + b;
        a = b;
        b = fib;
        count += 1;
    }
    fib
}