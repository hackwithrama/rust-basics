use std::thread::spawn;

pub fn test_threads(){
    let mut x = 0u128;
    for i in 0..500{
        x += i;
    }
    println!("\x1b[38;2;100;100;255mMain thread finished little bit of work, let's see what workers thread is doing!\x1b[0m");
}

pub fn test_spawn(){
    let thread_fn = || {
        let mut x = 0u128;
        for i in 0..500_000{
            x += i;
        }
        println!("Value of x : {x}");
    };
    println!("Starting new worker thread");
    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);
    println!("New worker thread stoped");

    // test_threads();
    loop{
        test_threads();
        if handle.is_finished() && handle2.is_finished(){
            println!("worker thread Finished");
            break;
        }
    }

    let _ = handle.join();
    let _ = handle2.join();

}