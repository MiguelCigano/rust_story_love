use std::io;

fn main() {
    let mut input = String::new();  // se crea una variable que almacena la entrada
    println!("Ingrese nombre: ");

    io::stdin()
        .read_line(&mut input)      // lee entrada
        .expect("Error al leer la linea");
    println!("valor 1, {}", input.trim());     // .trim es para eliminar saltos de línea.
}
