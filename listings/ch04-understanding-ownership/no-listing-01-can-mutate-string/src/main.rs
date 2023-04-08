fn main() {
    // ANCHOR: here
    let mut s = String::from("hola");

    s.push_str(", mundo!"); // push_str() agrega un literal a un String

    println!("{}", s); // Esto imprime "hola, mundo!"
                       // ANCHOR_END: here
}
