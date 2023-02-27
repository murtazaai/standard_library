use std::{sync::{mpsc::{Sender, Receiver}, Arc, Mutex}, thread, net::TcpListener};

use threadpool::ThreadPool;

const THREADS: i32 = 10;

pub fn send_message_on_channel(sender: Sender<i32>) {
    for i in 0..THREADS {
        let sender_clone = sender.clone();

        let x = thread::spawn(move || {
            let y = sender_clone.send(i).unwrap();
        });           
    }

}

pub fn receive_message_on_channel(receiver: Receiver<i32>) -> Vec<i32>{

    let mut receiver_ds: Vec<i32> = vec![];

    for i in 0..THREADS {
        receiver_ds.push(receiver.recv().unwrap());
    }

    receiver_ds
}

pub fn mutex_thread_handle_join() -> Arc<Mutex<i32>> {
    
    let arc = Arc::new(Mutex::new(0));

    for _ in 0..10 {

        let arc_clone = arc.clone();

        let join_handle = thread::spawn(move || {

            let mut number = arc_clone.lock().unwrap();                

            *number += 1;

        });
    }

    arc
}

pub fn thread_pool() {
    let thread_pool = ThreadPool::new(8);

    // let tcp_listener = 
    
    TcpListener::bind("localhost:9999".to_string()).unwrap();

    // for stream in tcp_listener.incoming() {
        
    //     let mut buf = "".to_string();

            // thread_pool.execute(|| {
            //     //     // stream.unwrap().read_to_string(&mut buf);
            // });
    // }
}