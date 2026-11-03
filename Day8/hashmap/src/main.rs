use std::collections::HashMap;

fn main() {
    //Basic of HashMap

    let mut scores = HashMap::new(); //create new HashMap

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let MxPern = String::from("Blue");
    let score = scores.get(&MxPern).copied().unwrap_or(0);
    println!("{MxPern}");
    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    
    //Managing Ownership in Hash Maps

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{map:?}");

    //Updating value based on old value

    let text = "hello world wonderful world";

    let mut maps = HashMap::new();

    for word in text.split_whitespace() {
        let count = maps.entry(word).or_insert(0);
        *count += 1;

        println!("{count}");
    }

}
