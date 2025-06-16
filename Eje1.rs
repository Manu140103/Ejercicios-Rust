use std::io;

fn main() {
    println!("Introduce un número:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer entrada");

    let number: f64 = input.trim().parse().expect("¡Debes ingresar un número!");
    let square = number * number;

    println!("El cuadrado de {} es {}", number, square);
}