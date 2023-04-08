fn main() {
    // ANCHOR: here
    let mut s = String::from("hola");

    {
        let r1 = &mut s;
    } // r1 se sale de su ambito aquí, por lo que no hay problema 
      // si creamos otra referencia mutable

    let r2 = &mut s;
    // ANCHOR_END: here
}
