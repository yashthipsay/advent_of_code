fn main() {
    let mut greeting2 = String::from("Greeting");
    print_greeting(&mut greeting2);
    let greeting = String::from("Hello");
    print_greeting(&greeting);
    print_greeting(&greeting);
}

fn print_greeting(message: &String){
    println!("Greeting: {}", message);
}

fn print_greeting2(message: &mut String){
    message.push_str("");
}
