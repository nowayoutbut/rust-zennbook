use std::collections::HashMap;

fn main() {
    // let mut v: Vec<u32> = vec![5, 6, 7];

    // let m = v.iter().map(|x| x * 2);

    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.entry("Yellow".to_string()).or_insert(50);

    scores.insert("Yellow".to_string(), 10);

    println!("{:?}", scores);
}
