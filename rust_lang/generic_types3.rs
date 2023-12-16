// struct Point {
//     x: f64,
//     y: f64,
// }

// impl Point{

//     fn origin() -> Point{
//         Point{x: 0.0, y: 0.0}
//     }

//     fn new(x: f64, y: f64) -> Point{
//         Point{x: x, y: y}
//     }
// }

// struct Rectangle {
//     p1: Point,
//     p2: Point,
// }

// impl Rectangle {
//     fn area(&self) -> f64 {
//         let Point {x: x1, y: y1} = self.p1;
//         let Point {x: x2, y: y2} = self.p2;

//         (x1-x2) * (y1-y2)
//     }

//     fn perimeter(&self) -> f64 {
//         let Point {x: x1, y: y1} = self.p1;
//         let Point {x: x2, y: y2} = self.p2;

//         2.0 * ((x1-x2) + (y1 - y2))
//     }

//     fn translate(&mutself, x: f64, y:64) {
//         self.p1.x = x;
//         self.p2.x = x;

//         self.p1.y = y;
//         self.p2.y = y;
//     }

    
// }


// -------------------------------------------------------

struct Sheep {}
struct Cow {}

trait Animal{
    fn animal(&self) -> &'static str;
}

impl Animal for Sheep {
    fn animal(&self) -> &'static str {
        "Sheep"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "Cow"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow{})
    }
}
