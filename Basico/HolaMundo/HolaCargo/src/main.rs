use std::io::Write;
use rand::Rng;

fn main() {
    // cuerpo de la función
    saludar_usuario();
}

fn saludar_usuario() -> Result<bool, String> {
    let nombre = recoger_nombre();
    let edad = edad_aleatoria();
    println!("¡Hola, {}! Tienes {} años.", nombre, edad);

    loop {
        print!("Todo es correcto? (s/n): ");
        std::io::stdout().flush().unwrap(); // Asegura que el mensaje se imprima antes de esperar la entrada

        let respuesta = s_n_verify();

        match respuesta {
            Ok(true) => {
                println!("¡Genial! ¡Gracias por usar el programa!");
                break Ok(true); // Sale del bucle si la respuesta es 's'
            }
            Ok(false) => {
                let nueva_edad = edad_aleatoria();
                println!("{} tu edad es de {} años? (s/n): ", nombre, nueva_edad);
                continue; // Vuelve a preguntar si la respuesta es 'n'
            }
            Err(e) => {
                println!("Error: {}", e);
                continue; // Vuelve a preguntar en caso de error
            }
        }
    }
}



fn recoger_nombre() -> String {
    let mensaje = "Por favor, introduce tu nombre: ";
    return recoger_teclado(mensaje);
}

fn edad_aleatoria() -> u8 {
    let mut random = rand::rng(); // Crea un generador de números aleatorios
    random.random_range(1..=100) // Número entre 1 y 100 (incluye el 100). Puede usarse u8 que comprende 1 a 255.
}

fn recoger_teclado(String: mensaje) -> String {
    print!("{}", mensaje);
    std::io::stdout().flush().unwrap();  // Asegura que el mensaje se imprima antes de esperar la entrada

    let entrada = std::io::stdin();

    let mut teclado = String::new();
    entrada
        .read_line(&mut teclado)
        .expect("Error al leer la línea");

    return teclado.trim().to_string();
}



fn s_n_verify() -> Result<bool, String> {
    let msn = "Por favor, responde con 's' o 'n': "; // 'msn' abreviacción de mensaje
    let respuesta_s_n = recoger_teclado(msn).to_lowercase();

    try {
        if respuesta_s_n == "s" {
            Ok(true); // Respuesta afirmativa
        } else if respuesta_s_n == "n" {
            Ok(false); // Respuesta negativa
        } else {
            println!("Respuesta no válida. Por favor, responde con 's' o 'n'.");
            return s_n_verify(); // Vuelve a preguntar si la respuesta no es válida
        }
    }
    catch e {
        Err: Err(("Error al procesar la respuesta: {}", e).to_string()) // Manejo de errores
    }
}