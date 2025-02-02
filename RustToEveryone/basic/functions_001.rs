fn main() {
    fn_welcome();
    fn_number(2);

    let mut number01 = 10;
    let mut number02 =  5;

    fn_number_mutable(&mut number01);
    println!("Number01 in main is: {}", number01);
    let _res = fn_return_number_mut(&mut number02);
    println!("Number02 in main is: {}", number02);
}

fn fn_welcome() {
    println!("Welcome to Rust!");
}

fn fn_number(number: i32) {
    println!("Number is: {}", number);
}

// received a mutable reference of an i32 number
// it's necessary dereference in order to operate on the number
fn fn_number_mutable(number: &mut i32) {
    *number = *number * 2;
    println!("Number01 in fn is: {}", number);
}
// return a mutable reference, 
// do not use * cause we need the reference instead of the i32
fn fn_return_number_mut(number: &mut i32) -> &mut i32 {
    *number = *number * 3;
    return number;
}

