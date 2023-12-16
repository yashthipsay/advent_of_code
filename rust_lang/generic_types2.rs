trait Area{
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(%self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64{
        self.width * self.height
    }
}

// ------------------------------------------------------------

// struct A;

// struct Single(A);

// struct SingleGen<T>(T);


// fn main() {
//     let _s = Single(A);

//     let _char: SingleGen<char> = SingleGen('a');
// }


// ------------------------------------------------------------

// struct A;
// struct S(A);
// struct SGen<T>(T);

// fn reg_fn(_s:S) {}


// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_t_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// fn main() {
//     reg_fn(S(A));
//     gen_spec_t(SGen(A));
//     gen_spec_i32(SGen(6));

//     generic::<char>(SGen('a'));

//     generic(SGen('a'));

//     generic(SGen('c'));
// }

// ------------------------------------------------------------


struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
}

