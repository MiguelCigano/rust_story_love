// this code is to work similar way to Template on C++
// only works with slice with lenth knew on compile time

fn print_array_values_rust<const N: usize>(my_array: &[u32; N]) {
    for value in my_array.iter() {
        println!("{}", value);
    }
}

fn main() {
    let my_array2 = [0u32, 0, 0, 0, 0];
    print_array_values_rust(&my_array2);

    let my_array3 = [1u32, 2, 3, 4];
    print_array_values_rust(&my_array3);
}
