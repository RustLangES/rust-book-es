fn main() {
    // ANCHOR: here
    let mut s = String::from("hola");

    let r1 = &s; // no hay problema
    let r2 = &s; // no hay problema
    let r3 = &mut s; // ¡ UN GRAN PROBLEMA !

    println!("{}, {}, y {}", r1, r2, r3);
    // ANCHOR_END: here
}
