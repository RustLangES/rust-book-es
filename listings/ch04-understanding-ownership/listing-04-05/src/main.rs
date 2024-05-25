fn main() {
    let s1 = String::from("hola");

    let (s2, len) = calcular_longitud(s1);

    println!("La longitud de '{s2}' es {len}.");
}

fn calcular_longitud(s: String) -> (String, usize) {
    let length = s.len(); // len() retorna la longitud de un String

    (s, length)
}
