#[derive(PartialEq, Debug)]
struct Car {color: String, motor: Transmission, roof: bool, age: (Age, u32)}

#[derive(PartialEq, Debug)]
enum Transmission {Manual, SemiAuto, Automatic}

#[derive(PartialEq, Debug)]
enum Age {New, Used}



fn car_quality (miles: u32) -> (Age, u32) {

    if miles > 0 {
        return (Age::Used, miles);
    }

    (Age::New, miles)
}

fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];


    let mut color = order as usize;
    if color > 4 {
        color = color - 4;
    }

    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {
        motor = Transmission::Automatic;
    } else if order % 3 == 0 {
        motor = Transmission::SemiAuto;
        roof = false;
    }

    Car {
        color: String::from(colors[(color-1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

fn main() {
    let mut order = 1;
    let mut car: Car;
}