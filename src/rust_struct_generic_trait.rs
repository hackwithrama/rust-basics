use std::cell::Cell;

#[derive(Debug)]
enum VehicleColor {
    Silver,
    Blue,
    Black,
    White,
    Green,
}

#[derive(Debug)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}

impl Vehicle {
    fn paint(&mut self, new_color: VehicleColor) {
        self.color = new_color
    }
    
    fn create_new_vehicle() -> Vehicle {
        let new_vehicle: Vehicle = Vehicle{
            manufacturer: "BMW".to_string(),
            model: "X1".to_string(),
            year: 2025,
            color: VehicleColor::Black,
        };
        new_vehicle
    }
}

pub fn test_create_static_new_vehicle() {
    let new_vehicle = Vehicle::create_new_vehicle();
    println!("{:?}", new_vehicle)
}

struct VehicleTuple(String, String, u16);

fn new_vehicle() -> Vehicle {
    let mut v1 = Vehicle {
        manufacturer: "Porsche".to_string(),
        model: "Panamara".to_string(),
        year: 2025,
        color: VehicleColor::Blue,
    };
    v1.paint(VehicleColor::Silver);
    v1
}

pub fn test_create_new_vehicle() {
    let result = new_vehicle();
    println!(
        "My new vehicle is {}-{}, and it is {}, {:?} color",
        result.manufacturer, result.model, result.model, result.color
    );
}

fn new_vehicle_tuple() -> VehicleTuple {
    VehicleTuple("Hyundai".to_string(), "Elantra".to_string(), 2025)
}

pub fn create_new_vehicle_tuple() {
    let my_vehicle_tuple = new_vehicle_tuple();
    println!("Manufacturer is {}, and model is {},{}", my_vehicle_tuple.0, my_vehicle_tuple.1, my_vehicle_tuple.2);
}


struct Person<'p> {
    first_name: Cell<&'p str>,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
    visited_europe: bool,
}

fn new_person() -> Person<'static> {
    let p1 = Person {
        first_name: Cell::from("Ramachandran"),
        last_name: "Ramakrishnan".to_string(),
        birth_year: 1991,
        birth_month: 10,
        visited_europe: false,
    };
    p1.first_name.set("Ramachandran");
    p1
}

pub fn test_create_new_person() {
    let result = new_person();
    println!(
        "My name is {} {}, my birth year and month is {},{}",
        result.first_name.get(), result.last_name, result.birth_year, result.birth_month
    );
}
