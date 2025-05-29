use std::io::Write;
use rand::Rng;

fn main() {
    // cuerpo de la función
    saludar_usuario();
}

fn saludar_usuario() {
    let nombre = recoger_nombre();
    let edad = edad_aleatoria();
    println!("¡Hola, {}! Tienes {} años.", nombre, edad);
}

fn recoger_nombre() -> String {
    let entrada = std::io::stdin();

    let mut nombre = String::new();
    print!("Por favor, introduce tu nombre: ");
    std::io::stdout().flush().unwrap();

    entrada
        .read_line(&mut nombre)
        .expect("Error al leer la línea");

    return nombre.trim().to_string();
}

fn edad_aleatoria() -> u8 {
    let mut random = rand::rng(); // Crea un generador de números aleatorios
    random.random_range(1..=100) // Número entre 1 y 100 (incluye el 100). Puede usarse u8 que comprende 1 a 255.
}