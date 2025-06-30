struct Person{
    first_name: String,
    last_name: String
}

pub fn test_closures() {
    let add = |a: i32, b: i32| {
        println!("{} + {} = {}", a, b, a+b);
        a + b
    };
    println!("add 1 is {}", add(1, 1));

    let mut p1 = Person{
        first_name: "Rama".to_string(),
        last_name: "Chandran".to_string()
    };
    
    let mut change_name = || p1.last_name = "Shiva".to_string();
    change_name();
    println!("add p1 is {}", p1.last_name);
}
