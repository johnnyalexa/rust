use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 30);

    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score of team {} is {}", team_name, score);

    for (key, value) in &scores {
        println!("Score of team {} is {}", key, value);
    }

    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Green"), 16);

    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Green")).or_insert(20);

    println!("{:?}", scores2);


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
