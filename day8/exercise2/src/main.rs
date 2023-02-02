
fn main() {
    println!("Hello, world!");

    let text = String::from("This awesome planet will soon replace eart as greatest place to ever live");
    println!("Original text: {text}");
    println!("Pig latin test: {}",convert_to_pig_latin(&text));
}



fn convert_to_pig_latin(txt: &String) -> String {
    let vowels = vec!["a","e","i","o","u"];
    let mut words: Vec<String> = Vec::new();
    let mut new_words: Vec<String> = Vec::new();
    let mut spaces: Vec<usize> = Vec::new();
    let mut i = 0;
    for c in txt.chars() {
        if c == ' ' {
            spaces.push(i);
        }
        i += 1;
    }
    i = 0;
    for a in spaces {
        words.push(txt[i..a].to_string());
        i=a+1;
    }
    for word in &words {
        let f = &word[0..1];

        if vowels.contains(&f) {
            new_words.push(format!("{word}-hay"));
        } else {
            let second = &word[1..word.len()];
            new_words.push(format!("{second}-{f}ay"));
        }
    }
    println!("words are {:?}", &words);
    println!("new_words are {:?}", &new_words);
    new_words.join(" ")
}

