use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // This counter is shared between threads, so we need to use Arc and Mutex to make it thread safe.
    // Arc stands for Atomic Reference Counted, 
    // and is a type that provides shared ownership of a value of type T, allocated in the heap.
    let counter = Arc::new(Mutex::new(0));

    // Just like in C++, we did pthread arraysOfThreads[5]
    // Syntax vec! is a macro that creates a vector, which is a growable array type.
    // To make it static, we could use let mut handles: Vec<thread::JoinHandle<()>> = vec![];
    let mut handles = vec![];
    
    // A basic for loop in Rust, with a range from 0 to 5.
    for _ in 0..5 {
        // The clone method takes a reference to the Arc<T> type and increments the reference count.
        // This has *shared* ownership of the value,
        // and the value will not be deallocated until all owners have gone out of scope, i.e. ARC "count"
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // The lock method on a Mutex<T> will block the current thread until it is able to acquire a lock on the mutex.
            let mut num = counter.lock().unwrap();

            // The lock method returns a MutexGuard, which is a smart pointer that implements Deref to point at our inner data.
            // Therefore, when using .lock() we essentially get a pointer to the value of the mutex.
            *num += 1;

            // If we did not derefence, we would try to add 1 to the MutexGuard, which is not allowed.
        });
        // We push the handle to the vector of handles.
        // This means that we spawn 5 threads, and push the handles to the vector.
        handles.push(handle);
    }

    // We join the threads, and unwrap the result.
    for handle in handles {
        handle.join()
              .unwrap();
    }
    // Recall that unwrap is a method on a Result<T, E>. 


    println!("Result: {}", counter.lock().unwrap());

}
