// ANCHOR: all
fn main() {
    // ANCHOR: here
    let s1 = String::from("hola");

    let len = calcular_longitud(&s1);
    // ANCHOR_END: here

    println!("La longitud de '{}' es {}.", s1, len);
}

fn calcular_longitud(s: &String) -> usize {
    s.len()
}
// ANCHOR_END: all
