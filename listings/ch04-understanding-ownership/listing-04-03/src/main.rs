fn main() {
    let s = String::from("hola");  // s aparece en el ámbito

    tomar_ownership(s);             // El valor de s se mueve a la función...
                                    // ... y ya no es valido aquí

    let x = 5;                      // x aparece en el ámbito

    hacer_una_copia(x);             // x deberia moverse a la función,
                                    // pero i32 implementa Copy, entonces es
    println!("{x}");                // valido aún despues de llamar a la función

} // Aquí termina el ámbito, x es destruido con drop. La memoria es liberada.
  // s ya no existia porque habia sido movido a la función.
  // Nada especial ocurre.

fn tomar_ownership(un_string: String) { // un_string aparece en el ámbito
    println!("{un_string}");
} // Aquí termina el ámbito, un_string es destruido con drop. 
  // La memoria es liberada.

fn hacer_una_copia(un_entero: i32) { // un_entero aparece en el ámbito
    println!("{un_entero}");
} // Aquí termina el ámbito, un_entero es destruido. Nada especial ocurre.
