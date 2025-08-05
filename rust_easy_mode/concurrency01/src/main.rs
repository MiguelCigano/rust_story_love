use std::thread;
use std::time::Duration;

fn main() {

    // thread::spawn( || {
    //     println!("Hello from thread 1");
    // });
    // thread::sleep(Duration::from_millis(1000));
    // println!("Hello from main");

    // handle of wrapper
    let join_handle = thread::spawn( || {
        thread::sleep(Duration::from_millis(2000));
        println!("Hello from thread 1");
    });
    println!("Hello from main");
    join_handle.join().unwrap(); // unwrapper usign handle

    println!("Finished all");
}
