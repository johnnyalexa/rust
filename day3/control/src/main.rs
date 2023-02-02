fn main() {
    let number = 6;

    if number % 4 == 0{
        println!("divisivble with 4");
    } else if number % 3 == 0 {
        println!("divisible with 3");
    } else {
        println!("not divisivle")
    }


    let condition = true;
    let nr = if condition {5} else {6};
    println!("nr is {nr}");
}
