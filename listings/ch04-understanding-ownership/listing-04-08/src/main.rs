fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word obtendrá el valor 5

    s.clear(); // esto "vacía" el String, dejando s igual a ""

    // word aún tiene el valor 5 aquí, pero s no tiene ningún contenido que
    // podamos usar de manera significativa con el valor 5 tenga sentido
    // ¡así que word ahora es totalmente inválido!
}
// ANCHOR_END: here
