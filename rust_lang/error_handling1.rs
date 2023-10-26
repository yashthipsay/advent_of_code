enum Option<T> {
    None, 
    Some(T),
}

fn main(){


    let a_number = Some(7);
    match a_number {
        Some(7) => println!("number 7"),
        _ => {},
    }


    let amount = vec!["one", "two", "three"];

    let first = amount.get(0);
    println!("first is {:?}", first);

    let second = amount.get(90);
    println!("second is {:?}", second);

    for &index in [0, 2, 90].iter(){
        match amount.get(index) {
            Some(&"one") => println!("It's one."),
            Some(amount_name) => println!("It's a {}.", amount_name),
            None => println!("There is no such amount."),
        }
    }

   

  assert_eq!(Some("one").unwrap_or("two"), "one")
}