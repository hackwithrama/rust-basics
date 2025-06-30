pub fn test_if() -> bool{
    let age_to_drive: u8 = 16u8;

    println!("Enter the person's age:");
    let myinput = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();
    let age = myinput.replace("\n", "").parse::<u8>().unwrap();
    if age >= age_to_drive {
        println!("Issuing driver's license, because they are old enough");
    }else{
        println!("You are not old enough to get driving license")
    }

    let drivers_license: bool = if age >= age_to_drive {true} else {false};
    drivers_license
}

pub fn test_while(){
    let age_to_drive: u8 = 16u8;
    let mut current_age: u8 = 0u8;

    while current_age < age_to_drive {
        println!("Waiting...");
        current_age += 1;
        if current_age == 6{
            println!("You have 6 years old!");
            break;
        }
    }
}

pub fn test_loop(){
    let mut x = 1;

    loop{
        println!("value of x is {}", x);
        if x > 5 {
            break;
        }
        x += 1;
    }
}

pub fn test_for(){
    let ages = [12, 24, 43, 54, 14, 23];
    let age_to_drive = 16;
    
    for x in ages{
        if x < age_to_drive{
            println!("You are not allowed to drive : {}", x);
        }else{
            println!("You are allowed to drive around : {}", x);
        }
    }
}