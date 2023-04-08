fn main() {
    let referencia_a_la_nada = colgar();
}

fn colgar() -> &String {
    let s = String::from("hola");

    &s
}
