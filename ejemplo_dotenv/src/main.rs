use dotenv::dotenv;
use std::env;

fn main() {
    // Leyendo el dotenv, si el archivo
    // .env existe, entonces lo utilizara como
    // variables de entorno, si no existe,
    // entonces intentara obtener las variables de entorno
    // desde el sistema operativo directamente
    dotenv().ok();

    // Ahora podemos leer las variables que se establecieron en el archivo .env como si fueran
    // variables de entorno, recuerda que si no fueron definidos en un .env, entonces intentara
    // obtener los valores de las variables de entorno en el Sistema Operativo.
    let usuario: String = env::var("USUARIO").expect("No se encuentra la variable de entorno USUARIO");
    let token: String = env::var("TOKEN").expect("No se encuentra la variable de entorno TOKEN");

    println!("Valor de la variable de entorno USUARIO = {}", usuario);
    println!("Valor de la variable de entorno TOKEN = {}", token);
}
