#[derive(PartialEq, Debug)]

struct Car{
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {Manual, SemiAuto, Automatic}

#[derive(PartialEq, Debug)]
enum Age {New, Used}


fn car_quality(miles: u32) -> (Age, u32){
    let quality: (Age, u32) = (Age::New, miles);

    return quality;
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car{
    let car: Car = Car{
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    };

    return car;
}

fn main(){
    let colors =["Blue", "Green", "Red", "Silver"];
    
    let mut car: Car;
    let mut engine = Transmission::Manual; 
}