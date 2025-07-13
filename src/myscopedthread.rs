use std::thread::{self, scope};

struct Person {
    first_name: String,
}

pub fn test_thread_variable() {
    let age = 34;
    let p1 = Person {
        first_name: "John".to_string(),
    };

    let print_age = || {
        println!("This is child closure");
        println!("THREAD: Hi from thread variable, age is {}", age);
        println!("THREAD: Hi and my first name is : {}\n", p1.first_name);
    };

    // let _result = thread::spawn(print_age).join();
    let _result = scope(|scope| {
        scope.spawn(print_age);
    });

    println!("Giving control back to main thread\n");

    println!("Hi from thread variable, age is {}", age);
    println!("Hi and my first name is : {}\n", p1.first_name);

    println!("Finished printing age");
}
