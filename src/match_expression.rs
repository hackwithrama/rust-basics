pub fn test_match_int() {
    let my_age: u16 = 35;
    match my_age {
        1..=35 => println!("Your  age is under 35 years"),
        _ => println!("Your are not in this category of age"),
    }
}

pub fn test_match_string() -> u32{
    let car_manufacturer = "Hyundai";

    // match car_manufacturer {
    //     "Hyundai" => println!("Your car manufacturer is Hyundai"),
    //     "Skoda" => println!("Your car manufacturer is Skoda"),
    //     "Maruti" => println!("Your car manufacturer is Maruti"),
    //     _ => println!("Manufacturer is not supported by this program"),
    // }

    match car_manufacturer {
        "Hyundai" => 200000,
        "Skoda" => 300000,
        _ => 0
    }
}

pub fn test_match_array(){
    
}