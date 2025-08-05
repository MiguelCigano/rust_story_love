fn foo(name: &mut String) {
    name.push_str(" Mtz");
    println!("name after push in fn: {}", name);
}

fn increment(number_copy: &mut i32) {
    *number_copy += 1;
    println!("num in fn: {}", number_copy);
}

fn increment_copy(mut number_copy: i32) {
    number_copy += 1;
    println!("num in fn copy: {}", number_copy);
}

fn main() {

    let mut number: i32 = 10;
    increment(&mut number);
    println!("num in main: {}", number);

    increment_copy(number);  // using numbers copy
    println!("num copy in main: {}", number);

    let mut name = String::from("Miguel");
    foo(&mut name);
    println!("name after push in main: {}", name);
}
