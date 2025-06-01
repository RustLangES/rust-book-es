fn main() {
    // ANCHOR: here
    let mut s = String::from("hola");

    let r1 = &s; // no hay problema
    let r2 = &s; // no hay problema
    let r3 = &mut s; // ¡ UN GRAN PROBLEMA !

    println!("{r1}, {r2}, y {r3}");
    // ANCHOR_END: here
}
