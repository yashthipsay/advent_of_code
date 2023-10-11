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
}