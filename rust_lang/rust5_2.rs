// fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
//     vector.push(String::from(value));
//     vector.get(vector.len() - 1).unwrap()
// }

// fn main() {
//     let name1 = "Joe";
//     let name2 = "Chris";
//     let name3 = "Anne";

//     let mut names = Vec::new();

//     assert_eq!("Joe", copy_and_return(&mut names, &name1));
//     assert_eq!("Chris", copy_and_return(&mut names, &name2));
//     assert_eq!("Anne", copy_and_return(&mut names, &name3));

//     assert_eq!(
//         names,
//         vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
//     )
// }


fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);

}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {x};

fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}