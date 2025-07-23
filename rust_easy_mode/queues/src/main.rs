use std::io;

struct Node {
    name: String,
    request_service: String,
    next: Option<Box<Node>>,
}

struct Bank {
    queue_long: u32,
    first: Option<Box<Node>>,
}

fn create_bank() ->Bank {
    Bank {
        queue_long: 0,
        first: None,
    }
}

fn main() {
    loop {
        match operation {
            println!("Operation you wish to perform");
            println!("1) add client");
            println!("5) Exit...");
            let mut operation = string::new();
            io::stdin()
                .read_line(&mut operation)
                .expect("Faild to read line");
        }
    }
    

}
