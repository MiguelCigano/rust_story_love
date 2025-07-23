//! # Documentación de Prueba
//! 
//! Esta documentación proporciona ejemplos y comparaciones sobre la conversión de tipos en Rust y C++.
//! 
//! ## Ejemplo de Función
//! 
//! La siguiente función suma dos números:
//! 
//! ```rust
//! fn plusplus(a: i32, b: i32) -> i32 {
//!     a + b
//! }
//! ```
//! 
//! ## Conversión de Tipos
//! 
//! El concepto de conversión de tipos (o "casteo") es diferente en Rust en comparación con C++.
//! 
//! ### Casteo en C++
//! 
//! 1. **Casteo implícito**: Ocurre automáticamente cuando el compilador puede convertir un tipo a otro sin perder información.
//! 2. **Casteo explícito**: Se utiliza para convertir entre tipos de forma intencionada, mediante:
//!    - `static_cast`
//!    - `dynamic_cast`
//!    - `const_cast`
//!    - `reinterpret_cast` (permite conversiones entre tipos que pueden no ser seguras).
//! 
//! ```cpp
//! int a = 30000;
//! short b = static_cast<short>(a); // Casteo explícito
//! ```
//! 
//! ### Conversión en Rust
//! 
//! En Rust, no se permite el casteo implícito entre tipos primitivos. Debes usar métodos específicos para convertir entre tipos. Las conversiones pueden ser seguras o pueden hacer que el programa entre en pánico si hay pérdida de datos.
//! 
//! #### Conversión Segura
//! 
//! Se realiza mediante métodos como `try_into()` o `as`. Por ejemplo:
//! 
//! ```rust
//! let a: i32 = 30000;
//! let b: i16 = a.try_into().expect("El valor no cabe en i16");
//! ```
//! 
//! #### Conversión Sin Verificar
//! 
//! Puede causar desbordamiento, así que no se recomienda si no se está seguro del rango:
//! 
//! ```rust
//! let c: i16 = a as i16; // No se recomienda si no se está seguro del rango
//! ```

fn main() {
    let a: i32;
    let x: i32 = 20000;
    let y: i16 = x.try_into().expect("valor no cabe en un tipo i16");
    println!("{y} ");

    another_function();
    a = plusplus(3, 3);
    println!("suma: {a}");
}

fn another_function() {
    println!("Hola desde otra funcion");
}

fn plusplus(a: i32, b: i32) -> i32 {
    a + b
}
