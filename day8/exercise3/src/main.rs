use std::collections::HashMap;
use std::io;


fn main() {

    let mut departments = HashMap::<String, Vec<String>>::new();

    let menu = String::from("Insert of list members from a department!
    - to add do: Add Sally to Enginnering
    - to retrieve do: Get Engineering
    - Anything else to quit");
    println!("{}", menu);
    loop {
        println!(": ");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();
        let mut spaces = Vec::new();
        let mut i = 0;
        for a in buffer.chars() {
            if a == ' ' {
                spaces.push(i);
            }
            i += 1;
        }

        let mut words = Vec::new();
        i = 0;
        for a in spaces {
            words.push(buffer[i..a].to_string());
            i=a+1;
        }
        words.push(buffer[i..].to_string());
        let command = words[0].to_string();

        match command.as_str() {
            "Add" => {
                let name = words[1].to_string();
                let department = words[3].to_string();
                Add(&mut departments, name, department);
            },
            "Get" => {
                let department = words[1].to_string();
                Get(&mut departments, department);
            },
            _ => break,
        }
    }
    println!("End of app!");
}



fn Add(deps: &mut HashMap<String, Vec<String>>, name: String, department: String) {
    let mut list = deps.entry(department).or_insert(Vec::new());
    list.push(name);
    list.sort();
}

fn Get(deps: &mut HashMap<String, Vec<String>>, department: String) {
    let list = deps.get(&department);
    match list {
        Some(l) => {
            println!("{:?}", l);
        },
        None => println!("Department not found"),
    }
}
