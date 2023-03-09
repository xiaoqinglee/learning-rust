use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn thread() {
    // Rust 标准库使用 1:1 线程实现，这代表程序的每一个语言级线程使用一个系统线程。

    // spawn() 对参数 F 的要求是  F: FnOnce() -> T, 这是个很宽松的要求.
    // 回忆闭包相关知识, fn Fn FnMut 都实现了 FnOnce.
    let join_handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} in spwaned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        return String::from("42");
    });
    for i in 1..5 {
        println!("number {} in spwaned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // will block until join_handle exits
    let result = join_handle.join().unwrap();
    assert_eq!(result, "42");
}

fn move_() {
    let v = vec![1, 2, 3];
    // 如果不加 move, 那么就会为 race condition 创造了条件, rust 编译器不能容忍.
    let join_handle = thread::spawn(move || {
        println!("vector: {:?}", v);
    });
    join_handle.join().unwrap();
}

fn mpsc_() {
    // A channel is said to be closed if either the transmitter or receiver half is dropped.

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            //send() consumed val
            tx.send(val).unwrap();
        }
        // tx is dropped here
    });
    // recv() will block until it receives a value
    // try_recv() is the non-blocking version recv()
    let first = rx.recv().unwrap();
    println!("first: {}", first);
    // after tx being dropped, the channel is said to be closed and next() of rx's iterator returns None.
    for another in rx {
        println!("another: {}", another);
    }
    // first: hi
    // another: from
    // another: the
    // another: thread
}

fn multiple_transmitters() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread 1"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("thread 2"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    // when all transmitters are dropped, the channel is closed and the loop ends.
    for received in rx {
        println!("received: {}", received);
    }
}

fn receive_from_closed_channel() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread 1"),
        ];
        for val in vals {
            tx.send(val).unwrap();
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("thread 2"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
        }
    });
    for _ in 1..=6 {
        rx.recv().unwrap();
    }
    // after 6 elements being sent, all transmitters are dropped and the channel is closed.
    thread::sleep(Duration::from_secs(1));
    // try to receive another element
    // 如果 channel 已经关闭, 那么 Receiver<T> 的 recv() 方法返回 Result::Err(RecvError)
    let another = rx.recv().unwrap(); //panic
}

fn send_to_closed_channel() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread 1"),
        ];
        for val in vals {
            // 如果 channel 已经关闭, 那么 Sender<T> 的 send(T) 方法返回 Result::Err(SendError<T>)
            tx.send(val).unwrap(); //panic
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("thread 2"),
        ];
        for val in vals {
            // 如果 channel 已经关闭, 那么 Sender<T> 的 send(T) 方法返回 Result::Err(SendError<T>)
            tx2.send(val).unwrap(); //panic
        }
    });
    drop(rx);
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    thread();
    move_();
    mpsc_();
    multiple_transmitters();
    // send_to_closed_channel();
    // receive_from_closed_channel();
}
