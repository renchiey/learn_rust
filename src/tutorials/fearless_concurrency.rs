use std::thread;
use std::time::Duration;
use std::sync::mpsc; // stands for multiple producers single receiver
use std::sync::{ Mutex, Arc };
use std::rc::Rc;

fn mutexes() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 66;
    }

    println!("m = {m:?}")
}

fn share_mutexes() {
    let counter = Arc::new(Mutex::new(0)); // Atomic Reference Counting
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

fn multi_message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hio"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

fn threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..6 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        println!("Heres a vector: {v:?}");
    });

    handle2.join().unwrap();
}

pub fn main() {
    // threads();

    // message_passing();

    // multi_message_passing();

    // mutexes();

    share_mutexes();
}
