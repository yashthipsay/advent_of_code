// fn main() {
//     let mut greeting2 = String::from("Greeting");
//     print_greeting(&mut greeting2);
//     let greeting = String::from("Hello");
//     print_greeting(&greeting);
//     print_greeting(&greeting);
// }

// fn print_greeting(message: &String){
//     println!("Greeting: {}", message);
// }

// fn print_greeting2(message: &mut String){
//     message.push_str("");
// }


fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// Error message:
// fn failed_borrow<'a>() {
//     let _a = 12;

//     let y: &'a i32 = &_x;
// }

fn main() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    
}