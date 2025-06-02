use std::io::Write;
use rand::Rng;

fn main() {
    let numero_secreto = numero_aleatorio();
    let mut intentos: u8 = 0;
    let mut entrada: u8;

    println!("¡Bienvenido al generador de números aleatorios!");

    loop {
        intentos += 1;
        
        print!("Por favor, ingresa un número entero positivo entre 1 y 100: ");
        std::io::stdout().flush().unwrap();
        
        
        entrada = match recoger_teclado().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada no válida, debe ser un número entero positivo entre 1 y 100.");
                continue;
            }
        };

        match entrada.cmp(&numero_secreto) {
            Ordering::less => println!("El número es mayor que {}", entrada),
            Ordering::greater => println!("El número es menor que {}", entrada),
            Ordering::equal => {
                println!("¡Felicidades! Has adivinado el número correcto: {}", numero_secreto);
                println!("Número de intentos: {}", intentos);
                break;
            }
        }
        /* 
        if entrada < 1 || entrada > 100 {
            println!("Número fuera de rango. Por favor, ingresa un número entre 1 y 100:");
            continue;
        }

        if entrada == numero_secreto {
            println!("¡Felicidades! Has adivinado el número correcto: {}", numero_secreto);
            println!("Número de intentos: {}", intentos);
            break;
        } else if entrada < numero_secreto {
            println!("El número es mayor que {}", entrada);
        } else {
            println!("El número es menor que {}", entrada);
        }
         */
    }

}

fn recoger_teclado() -> String {
    let entrada = std::io::stdin();

    let mut teclado = String::new();
    entrada
        .read_line(&mut teclado)
        .expect("Error al leer la línea");

    return teclado.trim().to_string()//.expect("Entrada no válida, debe ser un número entero positivo");
}

fn numero_aleatorio() -> u8 {
    let mut random = rand::rng(); // Crea un generador de números aleatorios
    random.random_range(1..=100) // Número entre 1 y 100 (incluye el 100). Puede usarse u8 que comprende 1 a 255.
}