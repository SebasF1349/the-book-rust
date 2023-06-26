use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // to create a new thread we call the `thread::spawn` function and pass it a closure containing the code we want to run in the new thread
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // take into account that we can't guarantee the order of the execution and that when the main thread finishes then the program shut down
    // an example of a possible output
    /*hi number 1 from the main thread!
    hi number 1 from the spawned thread!
    hi number 2 from the main thread!
    hi number 2 from the spawned thread!
    hi number 3 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the main thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
    */

    // if we need to wait for a thread to finish, we can save the return value of `thread::spawn` on a varible of type `JoinHandle` and then call the `join` method on it to wait
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    /*hi number 1 from the main thread!
    hi number 2 from the main thread!
    hi number 1 from the spawned thread!
    hi number 3 from the main thread!
    hi number 2 from the spawned thread!
    hi number 4 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
    hi number 6 from the spawned thread!
    hi number 7 from the spawned thread!
    hi number 8 from the spawned thread!
    hi number 9 from the spawned thread! */

    // if we move the `handle.join()` to before the for in the main thread, then the result will be:
    /*hi number 1 from the spawned thread!
    hi number 2 from the spawned thread!
    hi number 3 from the spawned thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
    hi number 6 from the spawned thread!
    hi number 7 from the spawned thread!
    hi number 8 from the spawned thread!
    hi number 9 from the spawned thread!
    hi number 1 from the main thread!
    hi number 2 from the main thread!
    hi number 3 from the main thread!
    hi number 4 from the main thread!
     */

    // we can use the `move` keyword with closured passed to `thread::spawn` to transfer ownership of values from one thread to another
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    // to send messages from a thread or actor to anothers, Rust's std lib provides an implementation of channels
    // a channel has two halves: a transmitter and a receiver
    let (tx, rx) = mpsc::channel();
    // mpsc: multiple producer, single consumer
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); //`tx.send()` returns a `Result<T, E>` as the receiver may have been dropped
    });
    let received = rx.recv().unwrap(); //`rx.recv()` blocks the main thread and wait until a value is sent down the channel, it returns a `Result<T, E>` as will return an error to signal that no more values will be coming (because the transmitter closed)
                                       // `rx.try_recv()` do the same but doesn't block the main thread, in case we want to do something while waiting.
    println!("Got: {}", received);

    // to have many transmitters to one receiver, we can clone the transmitter with `let tx1 = tx.clone();`

    // we can work with `rx` as an interator and wait for many messages using loops like this:
    for received in rx {
        println!("Got: {}", received);
    }

    // channels can only send values of a single type

    // Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time
    // to access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex's lock; the lock is a data structure that is part of the muted that keeps track of who currently has exclusive access to the data

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // we use `lock()` to acquire the lock; this will block the current thread so it can't do any work until it's our turn to have the lock
                                         // calling `lock` returns a smart pointer called `MutexGuard`, wrapped in a `LockResult`that we handled with the call to `unwrap`
                                         // this smart pointer has a `Drop` implementation that releases the lock automatically when a `MutexGuard` goes out of scope, so we don't risk forgettin to release the lock and blocking the muted from being used by other threads, because the lock release happens automatically
        *num = 6;
    }
    println!("m = {:?}", m);

    // Mutex has not the Copy trait, so it can't be moved between threads
    // we can't wrap it into a `Rc<T>` as Rc doesn't implement the `Send` trait, so we must use another type
    // `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations; the a stands for atomic, meaning it's an atomically reference counted type (atomics are an additional kind of concurrency primitive - see the documentation for `std::sync::atomic`)
    let counter = Arc::new(Mutex::new(0));
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
    println!("Result: {}", *counter.lock().unwrap()); // Result: 10

    // the `Send` marker (`std::marker`) trait indicates that ownership of values of the type implementing `Send` can be transferred between threads
    // almost all primitive types are `Send`, aside from raw pointers, so if you create a type composed entirely of `Send` types is automatically markes as `Send` as well

    // the `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads; in other words, any type `T` is `Sync` if `&T` is `Send`
    // primitive types are `Sync`, and types composed entirely of types that are `Sync` are also `Sync`.

    // manually implementing these traits involves implementing unsafe Rust code
}
