use std::thread;
use std::time::Duration;

fn main() {
    // handle is now a JoinHandle, which is a type that represents a handle to a spawned thread.
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
}

    /* 
    The join method on a handle blocks the current thread until the thread represented by the handle terminates.
    The unwrap method on a Result<T, E> will return the Ok value if the Result is an Ok variant, and will panic if the Result is an Err variant.
    So basically, T is Ok, and E is Err... And Result is just a built-in enum type with two enums, T or E. You may know this from advanced programming like
    
    fn readInputfromKattis () -> Result<String, io::Error> 
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input) <-- check if OK, if not, return Err
    }

    fn main() 
    {
        let inputforproblem = readInputfromKattis();
        match  inputforproblem {
            Ok(input) => {
                // do something with input
            }
            Err(e) => {
                // do something with error
            }
        }
    }    
}
    In Scala the equivalent is Either L or R. 
    
    */