fn main() {
    // One of the biggest selling points of Rust is its safety
    // Through a very strict type system Rust can prevent many
    // common mistakes, in particular when it comes to concurrency

    // Concurrent (and conflicting) memory modification is impossible in Rust
    // Data races are also impossible in Rust

    // Rust cannot prevent all concurrency issues, deadlocks
    // and race conditions are still possible (as they must be)

    // Let's look at the most basic example of concurrency

    // We can spawn a new thread using the std::thread::spawn function
    // The spawn function takes a closure that will be executed in the new thread
    let handle = std::thread::spawn(|| {
        // This is the code that will run in the new thread
        println!("Hello from the new thread!");
    });

    // We can wait for the thread to finish using the join method
    // Join returns a Result because the thread may panic
    handle.join().unwrap();

    // Getting data from threads is simple
    let handle = std::thread::spawn(|| {
        // We can return a value from a thread by returning it from the closure
        42
    });

    // We can get the value from the thread by calling join
    let result = handle.join().unwrap();
    println!("The result from the thread is: {}", result);

    // We can also pass data to the new thread
    let data = "Hey".to_string();
    // We use the move keyword to move the data into the new thread
    // The closure takes ownership of the data
    let handle = std::thread::spawn(move || data);

    // println!("I cannot have this anymore: {}", data);

    let result = handle.join().unwrap();
    println!("The result from the thread is: {}", result);

    // We can also use channels to communicate between threads

    let (tx, rx) = std::sync::mpsc::channel();

    let handle = std::thread::spawn(move || {
        let data = "Hey trough a channel".to_string();
        tx.send(data).unwrap();
    });

    let result = rx.recv().unwrap();
    println!("The message from the thread is: {}", result);
    handle.join().unwrap();

    // We can also use shared memory between threads

    // For this we need two smart pointers Arc and Mutex
    // Arc is the Atomic version of RC, it allows us to copy the Mutex between threads
    let data = std::sync::Arc::new(std::sync::Mutex::new(0));

    let mut handles = vec![];
    for _ in 0..100 {
        // We clone the Arc to give each thread a reference to the data
        let data = data.clone();
        let handle = std::thread::spawn(move || {
            // We lock the mutex to get access to the data
            // Note on the unwrap: Locking a mutex can fail if the mutex is poisoned
            // This happens when a thread panics while holding the lock
            // A really rare event, hence why most libraries that
            // implement locks don't even bother with it, since the caller can't do anything about it
            let mut data = data.lock().unwrap();
            *data += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // We can now access the data
    let data = data.lock().unwrap();
    println!("The data is: {}", *data);

    // So far this is not very different from other languages
    // Rust however encodes thread safety in the type system
    // In particular it uses two marker traits Send and Sync

    // These are automatically implemented by the compiler for types that conform to certain rules
    // For example, Send is implemented for types that are safe to send to another thread
    // This includes all primitive types.
    // Types that are not send may for example contain thread local data

    // Sync is implemented for types that are safe to share between threads
    // Meaning that a T is only Sync if &T is Send
    // This applies to all primitive types.
    // Any type that provides interior mutability that is not thread safe will not be Sync

    // In general Send any Sync can be summed up as:
    // A type is Send if it is safe to send it to another thread.
    // A type is Sync if it is safe to share between threads (T is Sync if and only if &T is Send).

    // Thanks to this we can use the type system to prevent many concurrency issues

    // Rc contains a non-atomic counter, so it is not thread safe. This means that
    // sharing it between threads is not safe. Notably, Rc is
    // let data = std::rc::Rc::new(0);
    // let d1 = data.clone();
    // let d2 = data.clone();
    // let j = std::thread::spawn(move || {
    //     println!("The value of d1 is: {}", d1);
    // });

    // There are lots of different types pointers used for concurrency that
    // allow mutability in a thread safe way
    // Smart Pointers such as Mutex and RwLock
    // Or atomic types
    // There is no point in going over them all. The Rust documentation is very good
    // All the sync documentation lives at: https://doc.rust-lang.org/std/sync/index.html
}
