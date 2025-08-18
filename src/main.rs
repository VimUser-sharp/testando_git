use crate::{clean_screen::clear, scope::logic};
use std::io::{self, Write};

mod clean_screen;
mod scope;

struct Person {
    name: String,
    age: u32,
    height: f32,
}

fn main() {
    let mut people: Person = Person {
        name: String::new(),
        age: 0,
        height: 0.0,
    };
    let mut input: String = String::new();

    print!("Digite seu nome: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut people.name)
        .expect("falha na entrada");
    clear();

    print!("Digite sua idade: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("falha na entrada");
    clear();

    people.age = input.trim().parse().expect("erro ao converter para int");

    print!("Digite sua altura: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("falha na entrada");
    clear();

    people.height = input.trim().parse::<f32>().unwrap_or(0.00);

    logic(&people.name);
}
