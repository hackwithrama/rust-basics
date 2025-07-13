use std::{
    ops::AddAssign,
    sync::Mutex,
    thread::{scope, spawn},
};

pub fn test_mutex() {
    let score = Mutex::new(0u16);

    // let unlocked_data = score.lock();
    // let mut data = unlocked_data.unwrap();
    // data.add_assign(5);

    // println!("The score is {data}");
    // drop(data);

    let my_func = || {
        println!("Thread 1 is waiting for mutex lock");
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            println!("Thread 1 is adding {i}")
        }
        println!("The score after thread 1 is {data}\n")
    };

    let my_func2 = || {
        println!("Thread 2 is waiting for mutex lock");
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            println!("Thread 2 is adding {i}");
        }
        println!("The score after thread 2 is {data}\n")
    };

    // let _result = spawn(my_func).join();
    let _result = scope(|scope| {
        let handle1 = scope.spawn(my_func).join();

        if handle1.is_err() {
            println!("Thread 1 had error, let's handle it here");
        }

        let handle2 = scope.spawn(my_func2).join();
    });

    println!("The score is {}", score.lock().unwrap());
}
