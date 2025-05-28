fn main() {
    // cuerpo de la función
    println!("¡Hola, mundo!");
}

fn saludar_usuario() {
    let nombre = recoger_nombre();
    println!("¡Hola, {}!", nombre);
}

fn recoger_nombre() {
    let mut nombre = String::new();
    println!("Por favor, introduce tu nombre:");

    std::io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer la línea");

    return nombre.trim().to_string();
}