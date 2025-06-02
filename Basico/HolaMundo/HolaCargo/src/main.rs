use std::io::Write;
use rand::Rng;

fn main() {
    // cuerpo de la función
    let _ = saludar_usuario();
}

fn saludar_usuario() -> Result<bool, String> {
    let mut primera_vuelta = true;
    let nombre = recoger_nombre();
    let mut edad = edad_aleatoria();

    println!("¡Hola, {}! Tienes {} años.", nombre, edad);

    print!("Todo es correcto? (s/n): ");
    std::io::stdout().flush().unwrap(); // Asegura que el mensaje se imprima antes de esperar la entrada
    
    loop {
        if !primera_vuelta {
            print!("{} tu edad es de {} años? (s/n): ", nombre, edad);
            std::io::stdout().flush().unwrap(); // Asegura que el mensaje se imprima antes de esperar la entrada
        } else {
            primera_vuelta = false; // Marca que ya no es la primera vuelta
        }

        let respuesta = s_n_verify();

        match respuesta {
            Ok(true) => {
                println!("¡Genial! ¡Gracias por usar el programa!");
                break Ok(true); // Sale del bucle si la respuesta es 's'
            }
            Ok(false) => {
                edad = edad_aleatoria();
                continue; // Vuelve a preguntar si la respuesta es 'n'
            }
            Err(e) => {
                println!("\nError: {}", e);
                continue; // Vuelve a preguntar en caso de error
            }
        }
    }
}



fn recoger_nombre() -> String {
    print!("Por favor, introduce tu nombre: ");
    std::io::stdout().flush().unwrap(); // Asegura que el mensaje se imprima antes de esperar la entrada
    return recoger_teclado();
}

fn edad_aleatoria() -> u8 {
    let mut random = rand::rng(); // Crea un generador de números aleatorios
    random.random_range(1..=100) // Número entre 1 y 100 (incluye el 100). Puede usarse u8 que comprende 1 a 255.
}

fn recoger_teclado() -> String {
    /* 
     *  print!("{}", mensaje);
     *  std::io::stdout().flush().unwrap();  // Asegura que el mensaje se imprima antes de esperar la entrada
     */
    let entrada = std::io::stdin();

    let mut teclado = String::new();
    entrada
        .read_line(&mut teclado)
        .expect("Error al leer la línea");

    return teclado.trim().to_string();
}



fn s_n_verify() -> Result<bool, String> {
    let respuesta_s_n = recoger_teclado().to_lowercase();

    match respuesta_s_n.as_str() {
        "s" => Ok(true), // Respuesta afirmativa
        "n" => Ok(false), // Respuesta negativa
        /* else {
            println!("Respuesta no válida. Por favor, responde con 's' o 'n'.");
            //return s_n_verify(); // Vuelve a preguntar si la respuesta no es válida
        } */

        _ => Err("Error al procesar la respuesta: No has ingresado 's' o 'n'\n".to_string()) // Manejo de errores
    }
}