fn main() {
    let nome = String::from("Rust");
    passando(&nome);
    print!("{}", nome);
}

fn passando(nome: &String) {
    print!("{}", nome);
}
