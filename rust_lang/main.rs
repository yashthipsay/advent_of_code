fn student(message: &str){
    println!("\n{}", message);
}

fn main(){
    let a = "Rust";
    let mut number = 10;
    println!("Hello, World!");
    println!("Programming - {}", a);

    let number2: u32 =  15;

    struct Student {
        name: String,
        age: u8,
    }
    let user_1 = Student{name: String::from("John"), age: 25};
    let user_2 = Student{name: String::from("Jane"), age: 20};

    enum WebEvent{
        WeLoad,
        WEKeys(String, char),
        WEClick{x: i64, y: i64},
    }

    struct KeyPress(String, char);
    struct MouseClick{x: i64, y: i64};
    
    enum WebEvent{WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress)}
}