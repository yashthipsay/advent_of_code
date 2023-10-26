fn main() {
    let a = String::from("owighownbw");
    let b = String::from("xcbowh");

    let result = longest_word(&a, &b);
    println!("{}", result);

}

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String{
    if x.len() >y.len(){
        x
    } else {
        y
    }
}