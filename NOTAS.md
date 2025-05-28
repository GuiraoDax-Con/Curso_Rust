# Notas sobre contenido que considero importante sobre Rust y Cargo

## COMANDOS UTILES

- Ejecutar el archivo:

    ```Rust
        rustc nombre_archivo.rs
    ```

- Compila el proyecto en modo de "release" (lanzamiento). Esto significa que el compilador aplica optimizaciones para que el programa sea más rápido y eficiente, a diferencia del modo por defecto (debug), que prioriza la velocidad de compilación y la facilidad de depuración. El ejecutable generado suele ser más pequeño y rápido, ideal para distribución o producción.

    ```Cargo
        cargo build --release
    ```
