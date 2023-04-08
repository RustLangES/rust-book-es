fn main() {
    // ANCHOR: here
    {                      // s no es valido aquí, aún no está declarado
        let s = "hola";   // s es valido desde aquí

        // Hacer algo con s
    }                      // este ambito termina aquí, s ya no es valido
    // ANCHOR_END: here
}