enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter,
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quartar => 25,
    }
}

enum USState {
    Alabama,
    Alaska,
}

enum USCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quartar(state) => {
            println!("State quartar from {:?}!", state);    
        }
    }
}