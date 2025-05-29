fn main() {
    // cuerpo de la función
    saludar_usuario();
}

fn saludar_usuario() {
    let nombre = recoger_nombre();
    println!("¡Hola, {}!", nombre);
}

fn recoger_nombre() -> String {
    let entrada = std::io::stdin();

    let mut nombre = String::new();
    println!("Por favor, introduce tu nombre:");

    entrada
        .read_line(&mut nombre)
        .expect("Error al leer la línea");

    return nombre.trim().to_string();
}