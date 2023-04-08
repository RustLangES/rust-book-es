fn main() {
    // ANCHOR: here
    {
        let s = String::from("hola"); // s es valido desde aquí

        // Hacer algo con s
    }                                  // este ambito termina aquí, 
                                       // s ya no es valido
    // ANCHOR_END: here
}
