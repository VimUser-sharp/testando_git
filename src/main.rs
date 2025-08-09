use crate::scope::logic;
use std::io;

mod scope;

fn main() {
    let mut name = String::new();

    //print!("Digite seu nome: ");
    io::stdin().read_line(&mut name).expect("falha na entrada");
    logic(&name);
}
