# Notas sobre contenido que considero importante sobre Rust y Cargo

## Repositorios

[Paquetes para Cargo](https://crates.io/)

## COMANDOS UTILES

### Comandos Rust

- Ejecutar el archivo Rust directamente:

    ```Rust
        rustc nombre_archivo.rs
    ```

### Comandos Cargo

- Crear un nuevo proyecto Cargo

    ```Rust
        cargo new nombre_proyecto 
    ```

- Ejecutar el proyecto Cargo

    ```Rust
        cargo run   #Ejecuta el 'main.rs' en la carpeta 'src' del proyecto.
    ```

    ```Rust
        /ProyectoCargo
            |- /src
                |- main.sr -> ' El archivo princimpal del proyecto.'
            |- /target      -> ' Aparece despues de haber sido ejecutado por primera vez el proyecto.'
            |- Cargo.lock   -> ' Aparece despues de haber sido ejecutado por primera vez el proyecto.'
            |- Cargo.toml  -> ' Archivo de configuración del proyecto, en este se indican las librerias a usar.' Cargo ' se encarga de descargar e importar automaticamente las librerias o eso cro.'
    ```

- Comprobar si el proyecto tiene errores de compilación lo podemos comprobar con el comando:

    ```Rust
        cargo check
    ```

- Compila el proyecto en modo de "release" (lanzamiento).
Esto significa que el compilador aplica optimizaciones para que el programa sea más rápido y eficiente, a diferencia del modo por defecto, que prioriza la velocidad de compilación y la facilidad de depuración. El ejecutable generado suele ser más pequeño y rápido, ideal para distribución o producción. ¡¡¡Ojo, tarda más!!!

    ```Rust
        cargo build --release
    ```

## Librerias utiles

- Salto evitar salto de linea:

    ```Rust
        // Importación
        use std::io::Write;

        // Comando justo despues del print!
        std::io::stdout().flush().unwrap();
    ```

- Numero aleatorio:
- - Añadir a las dependencias de 'Cargo.toml':

    ```Rust
        rand = "0.9.1"
    ```

- - Coando en el archivo '.rs':

    ```Rust
        // Importar
        use rand::Rng;

        // Comandos
        let mut random = rand::rng(); // Crea un generador de números aleatorios
        random.random_range(1..=100) // Número entre 1 y 100 (incluye el 100) Puede usarse u8 que comprende 1 a 255.
    ```

- - Poner color al texto de la terminal:

    ```Rust
        // Importar
        use colored::Colorize;

        // Comando despues del texto
        .nombre_color() // Color del texto
        println!("Hola".green());

        .on_nombre_color() // Color de fondo del texto, como subrallado
        println!("Hola".on_green());
    ```
