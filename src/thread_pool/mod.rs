use std::{sync::mpsc::{Sender, Receiver}, thread};

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