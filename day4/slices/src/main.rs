fn main() {
    let mut a =String::from("Hello world morning new!");
    let x = first_word(&a[0..6]);
    let y = first_word(&a[7..11]);
    //a.clear();
    println!("x = {}", x);
    println!("y = {}", y);

   // let y = second_word(&a);

   let b = [1,2,3,4,5];
   let slice = &b[1..3];
   assert_eq!(slice, &b[1..3]);
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

//fn second_word(sL &String) -> &str {

//}