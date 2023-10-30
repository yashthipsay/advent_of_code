fn main(){
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("Numbers"),

        _=> println!("default value"),
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    let triple = (0, -2, 3);

    println!("This is a {:?}", triple);

    match triple {
        (0, y, z) => println!()
    }
}