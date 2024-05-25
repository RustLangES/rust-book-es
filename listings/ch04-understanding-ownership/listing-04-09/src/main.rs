// ANCHOR: here
fn first_word(s: &str) -> &str {
    // ANCHOR_END: here
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: usage
fn main() {
    let my_string = String::from("hello world");

    // `first_word` funciona con slices de un string, sean parciales o completos.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` también funciona con referencias de un string, que son equivalentes
    // a un slice completo de un String
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` funciona con slices de string literales, sean parciales o completos
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Por que los strings literales son slices de strings,esto también funciona,
    // sin necesidad de usar la sintaxis de slices.
    let word = first_word(my_string_literal);
}
// ANCHOR_END: usage
