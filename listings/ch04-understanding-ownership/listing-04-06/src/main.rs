fn main() {
    let s = String::from("hola");

    modificar(&s);
}

fn modificar(un_string: &String) {
    un_string.push_str(", mundo");
}
