use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Hello, world!");
}


mod math {
    type Complex = (f64, f64);
    pub fn sin(f: f64){ /* */}
    pub fn cos(f: f64){}
    pub fn tan(f: f64){}
}
