fn main() {
    println!("Hello, world!");

    let x = 5;
    println!("The values of x is: {x}");

    let x = x + 1;
    {
        let x = x + 5;
        println!("The value of x is {x}");
    }

    println!("The value of x is {x}");

    const THRESS_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let spaces = "     ";
    let spaces = spaces.len();



    let x = 2.0;
    let y: f32 = 3.0;


    let sum = 5 + 1_0;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    let tup = (500, 6.4, 1);

    let (_, y, _) = tup;

    println!("the value of y is: {y}");
    println!("the float value is: {}",tup.1);


    let a = [1, 2, 3, 4, 5];

    println!("4th element is: {}", a[3]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("3rd element is: {}", a[2]);
}
