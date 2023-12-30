use std::fmt::Error;
use std::env;


fn main() -> () {
    let input: Vec<String> = env::args().collect();

    let mut sum: i32 = 0;

    let cal_val: Vec<i32> = input.iter().map(|x| {
      let first_digit = x.chars().filter(|c| c.is_numeric()).next().unwrap();
      let last_digit = x.chars().filter(|c| c.is_numeric()).last().unwrap();
      let number = String::from(first_digit.to_string()) + &String::from(last_digit.to_string());
      number.parse::<i32>().unwrap()
        }).collect();

        sum = cal_val.iter().sum();

       println!("{:?}", sum);
    
    
}