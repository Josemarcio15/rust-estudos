fn main() {
    let a: u32 = 10;

    let b: u64 = a; //ERRADO
    let b: u64 = a as u64; //CORRETO

    print!("{}", b)
}
