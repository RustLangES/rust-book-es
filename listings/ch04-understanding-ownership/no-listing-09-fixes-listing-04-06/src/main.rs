fn main() {
    let mut s = String::from("hola");

    modificar(&mut s);
}

fn modificar(un_string: &mut String) {
    un_string.push_str(", mundo");
}
