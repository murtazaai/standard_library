use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};
use std::sync::{Arc, Barrier, mpsc, Mutex};

use negative_impl::negative_impl;

const THREADS: i32 = 10;

#[allow(warnings)]
pub fn send_message_on_channel(sender: Sender<i32>) {
    for i in 0..THREADS {
        let sender_clone = sender.clone();

        let x = thread::spawn(move || {
            let y = sender_clone.send(i).unwrap();
        });
    }
}

#[allow(warnings)]
pub fn receive_message_on_channel(receiver: Receiver<i32>) -> Vec<i32> {
    let mut receiver_ds: Vec<i32> = vec![];

    for i in 0..THREADS {
        receiver_ds.push(receiver.recv().unwrap());
    }

    receiver_ds
}

struct Thread {}

#[negative_impl]
impl ! Send for Thread {}

#[negative_impl]
impl ! Sync for Thread {}

#[allow(dead_code)]
fn func<T: ?Sized>(_t: &T) {}

pub fn thread_spawn() {
    let vector = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector: {:?}", vector);
    });

    let result = handle.join();

    println!("Result: {:?}", result);

    println!("Main thread");
}


fn timer(d: i32, tx: mpsc::Sender<i32>) {
    thread::spawn(move || {
        println!("{} sent!", d);
        tx.send(d).unwrap();
    });
}

#[allow(dead_code)]
pub fn channels() {
    let (tx, rx) = mpsc::channel();

    for i in 0..5 {
        timer(i, tx.clone());
    }

    drop(tx);

    for received_value in rx {
        println!("Received value: {}", received_value);
    }
}

#[allow(dead_code)]
pub fn threads_with_locking() {
    let mutex = Mutex::new(10);


    let mut number = mutex.lock().unwrap();
    *number = 20;
    drop(number);

    let mut number_1 = mutex.lock().unwrap();
    *number_1 = 40;
    drop(number_1);
}

#[allow(dead_code)]
pub fn multithreading_with_mutex() -> i32 {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut number = counter.lock().unwrap();
            *number += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let counter_1 = counter.lock().unwrap().clone();

    counter_1
}

#[allow(dead_code)]
pub fn multithreading_with_barriers() {
    let mut threads = vec![];

    let barrier = Arc::new(Barrier::new(3));

    let data = Arc::new(vec![
        vec![1, 2, 3, 4, 5, 6],
        vec![1, 2, 3, 4, 5, 6],
        vec![1, 2, 3, 4, 5, 6],
    ]);

    let result = Arc::new(Mutex::new(0));

    for i in 0..3 {
        let barrier = barrier.clone();
        let data = data.clone();
        let result = result.clone();
        let handle = thread::spawn(move || {
            let a = data[i][0..3].iter().sum::<i32>();

            *result.lock().unwrap() += a;

            println!("Thread {} part 1 is done", i);

            barrier.wait();

            let a = data[i][3..6].iter().sum::<i32>();

            *result.lock().unwrap() += a;

            println!("Thread {} is complete", i);
        });

        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }

    println!("Final result is {}", result.lock().unwrap());
}

pub fn scoped_threads() {
    let mut vector = vec![1, 2, 3];

    let mut a = 0;

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("This is the first scope");
            println!("Vector: {:?}", vector);
        });

        scope.spawn(|| {
            println!("This is the second scope");
            a += 20;
        });
    });

    println!("Threads are now complete");

    vector.push(10);
    println!("a: {}, vector: {:?}", a, vector);
}

#[allow(dead_code)]
async fn printing() {
    println!("This is an async function");
}

#[allow(dead_code)]
pub async fn async_call() {
    let a = printing();

    println!("Function has not been polled yet");

    drop(a);

    //a.await;
}

#[allow(dead_code)]
async fn complex_function() {
    println!("Function 1 started");

    for _ in 0..100_0000_000 {}

    println!("Function 1 ended");
}

#[allow(dead_code)]
async fn simple_function() {
    println!("Function 2 started");
}

// #[allow(dead_code)]
// pub async fn tasks() {
//     tokio::select! {
//         // Winner thread is completed first which is simple_function
//         _ = complex_function() => println!("Complex function completed first");
//         _ = simple_function() => println!("Simple function completed first");
//     }
// }