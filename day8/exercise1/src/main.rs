use std::collections::HashMap;



fn main() {
    println!("Hello, world!");

    let mut lst = vec!(1,5,87,43,78,34,67,23,67,6,5,3,7,3,7,8,67);
    println!("{:?}", lst);

    lst.sort();
    println!("{:?} of size {}", lst, lst.len());

    println!("Median is element {} of value {}", lst.len()/2, lst[lst.len()/2 - 1]);


    let mut map = HashMap::new();
    for i in lst {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("Map is {:?}", map);

    let mut v = 0;
    let mut k = 0;
    for (key, value) in map {
        if value > v {
            v = value;
            k = key;
        }
    }
    println!("Most often we find the value {} , {} times", k, v);
}


