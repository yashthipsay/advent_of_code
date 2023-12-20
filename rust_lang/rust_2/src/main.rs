use std::collections::HashMap;

fn hashmap(){
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50); 

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
}

fn main() {

}