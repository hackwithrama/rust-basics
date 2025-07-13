use std::{sync::mpsc, time::Duration};

pub fn test_channels() {
    let (tx, rx) = mpsc::channel::<u8>();

    let send_result1 = tx.send(250);
    let send_result2 = tx.send(150);

    // println!("Send 1 status is : {}", send_result1.is_ok());
    // println!("Send 2 status is : {}", send_result2.is_ok());

    // let receive_result = rx.recv_timeout(Duration::from_millis(300));
    // println!("receive status is : {}", receive_result.is_ok());
    // println!("receive dat is : {}", receive_result.unwrap());

    // let receive_result = rx.recv_timeout(Duration::from_millis(300));
    // println!("receive status is : {}", receive_result.is_ok());
    // println!("receive dat is : {}", receive_result.unwrap());
}
