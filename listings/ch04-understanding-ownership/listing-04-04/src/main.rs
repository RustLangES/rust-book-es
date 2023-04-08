fn main() {
    let s1 = da_un_ownership();         // da_un_ownership es llamado y
                                        // devuelve el valor de retorno
                                        // a s1

    let s2 = String::from("hola");     // s2 aparece en el ambito

    let s3 = toma_y_devuelve(s2);  // s2 es movido a la función
                                        // toma_y_devuelve, que también
                                        // retorna el valor de s2 a s3
} // Fin el ambito, s3 es destruido con drop y se libera la memoria. 
  // s2 fue movido previamente, entonces no pasa nada. 
  // s1 es destruido con drop y se libera la memoria.

fn da_un_ownership() -> String {             // da_un_ownership mueve su
                                             // retorno a la función que la
                                             // llama

    let un_string = String::from("tuyo");    // un_string aparece en el ambito

    un_string                                // un_string es retornado y
                                             // mueve su valor
}

// Esta función toma un String y devuelve uno
fn toma_y_devuelve(un_string: String) -> String { // un_string aparece 
                                                  // en el ambito

    un_string  // un_string es retornado y mueve su valor
}
