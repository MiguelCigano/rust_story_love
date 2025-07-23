fn main() {
    println!("Hello, world!");
}

struct Movies {
    title : String,
    list: Vec<String>,
}

impl Movies {
    fn new(title: $str) -> Self {
        Movies {
            title: title.to_string(), 
            list: Vec::new(),
        }
    }
}