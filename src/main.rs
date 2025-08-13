use crate::{clean_screen::clear, scope::logic};
use std::io::{self, Write};

mod clean_screen;
mod scope;

fn main() {
    let mut name: String = String::new();

    print!("Digite seu nome: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("falha na entrada");
    clear();

    logic(&name);
}
