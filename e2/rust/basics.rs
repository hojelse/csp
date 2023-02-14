// I am new to Rust, so this will definitely not be as standard as in C++ or C!

use std::thread;
use std::time::Duration;

fn main() {
    let _integer : i8 = 0; // Rust has type inference, but you can also declare types explicitly. 
    let mut _integer_mut : i8 = 0; // Rust has immutable variables by default, so you have to declare them as mutable if you want to change them.

    /* 
    In Rust, ownership is a big deal! For example, to pass a variable to a function, you have to pass it by reference, as we talked about.
    In the below snippet, I can not print out integer with the println! macro, as it is not owned by the main thread. 
    The move keyword is used to force the closure to take ownership of the values it uses in the environment.
    */

            // TAKE OWNERSHIP OF INTEGER
    thread::spawn(move|| { // <-- this is a closure, not exactly a lambda function, but similar, can force ownership of variables)
        for _i in 1..10    // In Rust, if you declare an iterator as i, you will get a compiler warning. Use _ instead or i_.
        { 
            _integer_mut += 1; 
            // _integer_mut += _; NOT ALLOWED, only LHS can be _ 
            
            println!("Wow, I can see {}!", _integer_mut);
            // We add thread sleep to make sure the main thread finishes first, otherwise the main thread will be terminated before the child thread finishes.
            thread::sleep(Duration::from_millis(1));
        }
    });

    // This is the main thread
    for i in 1..6 { 
        println!("hi number {} from the main thread!", i); 
        thread::sleep(Duration::from_millis(1));
    }
    
}
