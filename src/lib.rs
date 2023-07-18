use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub fn moving_data_to_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector from thread: {:?}", v);
        return v;
    });

    let v = handle.join().unwrap();

    println!("Here's a vector from main: {:?}", v);
}

pub fn channels() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
        }
    });

    for received in rx {
        println!("Got: {}", received)
    }
}

pub fn shared_state_concurrency() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..1000 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
            println!("{}", *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
