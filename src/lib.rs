use std::thread;

pub fn moving_data_to_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector from thread: {:?}", v);
        return v;
    });

    let v = handle.join().unwrap();

    println!("Here's a vector from main: {:?}", v);
}
