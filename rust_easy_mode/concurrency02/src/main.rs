use std::thread;
use std::time::Duration;

fn main() {
    
    let name = String::from("Rusty");
    println!("Hello {} from main 1", name);

    let join_handle = thread::spawn ( move || {
        println!("Waiting...");
        thread::sleep(Duration::from_millis(1000));
        println!("{} is deleted!", name);
    });
    
    println!("Hello {} from main 2", name);
    join_handle.join().unwrap();
    println!("Finished all");
}
