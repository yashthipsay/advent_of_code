use std::fmt;



#[derive(Debug, PartialEq)]
struct A<T> {
    x: T,
    y: T
}



fn main() {

    impl fmt::Display for A {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let p1 = A {x: 1, y:2};
    let p2 = A {x:4, y:-3};

    if p1==p2 {
        println!("yes");
    } else {
        println!("no");
    }

    println!("{:?}", p1);
    println!("{}", p1);
}

