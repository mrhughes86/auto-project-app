enum VehicleType {
    Infiniti,
    Toyota,
    Mercedes,
    Audi,
}

impl VehicleType {
    fn print(&self) {
        match self {
            VehicleType::Infiniti => println!("Vehicle: Infiniti"),
            VehicleType::Toyota => println!("Vehicle: Toyota"),
            VehicleType::Mercedes => println!("Vehicle: Mercedes"),
            VehicleType::Audi => println!("Vehicle: Audi"),
        }
    }
}
enum VehicleColor {
    Red,
    Blue,
    Black,
    Grey,
}

impl VehicleColor {
    fn print(&self) {
        match self {
            VehicleColor::Red => println!("Color: Red"),
            VehicleColor::Blue => println!("Color: Blue"),
            VehicleColor::Black => println!("Color: Black"),
            VehicleColor::Grey => println!("Color: Grey"),
        }
    }
}

enum Transmission {
    Automatic,
    Standard,
}

impl Transmission {
    fn print(&self) {
        match self {
            Transmission::Automatic => println!("Transmission: Automatic"),
            Transmission::Standard => println!("Transmission: Standard"),
        }
    }
}

struct Vehicle {
    car_type: VehicleType,
    car_color: VehicleColor,
    trans: Transmission,
    price: f64,
}

impl Vehicle {
    fn new(
        price: f64,
        car_type: VehicleType,
        car_color: VehicleColor,
        trans: Transmission,
    ) -> Self {
        Self {
            price,
            car_type,
            car_color,
            trans,
        }
    }
    fn print(&self) {
        self.car_type.print();
        self.car_color.print();
        self.trans.print();
        println!("Price: {:?}", self.price);
    }
}

fn main() {
    let my_vehicle = Vehicle::new(
        24400.00,
        VehicleType::Infiniti,
        VehicleColor::Black,
        Transmission::Standard,
    );
    let my_other_vehicle = Vehicle::new(
        24400.00,
        VehicleType::Audi,
        VehicleColor::Blue,
        Transmission::Automatic,
    );
    my_vehicle.print();
}
