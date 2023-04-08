fn main() {
    let s1 = String::from("hola");

    let len = calcular_longitud(&s1);

    println!("La longitud de '{}' es {}.", s1, len);
}

// ANCHOR: here
fn calcular_longitud(s: &String) -> usize { // es una referencia a un String
    s.len()
} // Aquí, s sale de ámbito. Pero como no tiene el ownership/la propiedad sino 
  // que s es solo un prestamo, no se destruye, se regresa al propietario, s1.
// ANCHOR_END: here
