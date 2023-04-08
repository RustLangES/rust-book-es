fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // no hay problema
    let r2 = &s; // no hay problema
    println!("{} y {}", r1, r2);
    // variables r1 y r2 no se usaran más a partir de aquí

    let r3 = &mut s; // no hay problema
    println!("{}", r3);
    // ANCHOR_END: here
}
