fn main() {
    let referencia_a_la_nada = colgar();
}

// ANCHOR: here
fn colgar() -> &String { // colgar retorna una referencia a un String

    let s = String::from("hola"); // s es un nuevo String

    &s // retornamos una referencia a la String, s
} // Aquí, s sale de ámbito y se libera su memoria. 
  // ¡Pero retornamos una referencia a ella!
  // ¡Peligro! ¡Esta referencia apunta a memoria que ya no existe!
// ANCHOR_END: here
