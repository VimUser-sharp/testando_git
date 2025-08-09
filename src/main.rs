use crate::scope::logic;
use std::io::{self, Write};

mod scope;

fn main() {
    print!("Digite seu nome: ");
    io::stdout().flush().unwrap();
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("falha na entrada");
    logic(&name);
}
