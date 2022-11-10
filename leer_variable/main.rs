use std::env;

fn sumar(a: usize) -> Result<usize, env::VarError> {
    // Leyendo la variable de entorno, en caso no se haya creado el programa terminara con error mostrando el mensaje del expect
    let valor_a_sumar = env::var("VALOR_A_SUMAR").expect("Falta Definir la Variable de Entorno VALOR_A_SUMAR");

    // Por defecto, las variables de entorno se leen como String, por lo que para la funcion hay que hacer el cast explicito a usize
    Ok(a + valor_a_sumar.parse::<usize>().unwrap())
}

fn main() {
    // Por facilidad, no se valida el input. Recomendable hacerlo para produccion
    let resultado: usize = sumar(1).unwrap();
    println!("resultado suma = {}", resultado);
}
