use random::{Source, Xorshift128Plus};

fn main() {
    // Inicializar el generador con una semilla
    let seed = [42, 12345]; // Una semilla de dos valores
    let mut source = Xorshift128Plus::new(seed);

    // Generar un número aleatorio de tipo i32
    let num: i32 = source.read();

    // Imprimir el resultado
    println!("Hello, world! {}", num);
}