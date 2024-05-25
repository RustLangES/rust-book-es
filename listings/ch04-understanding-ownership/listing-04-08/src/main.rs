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

    // word aún tiene el valor 5 aquí, pero ya no hay un string para que
    // usar el valor 5 tenga sentido, ¡word es totalmente invalida!
}
// ANCHOR_END: here
