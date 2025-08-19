use crate::{clean_screen::clear, scope::logic};
use std::io::{self, Write};

mod clean_screen;
mod scope;

struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut people: Person = Person {
        name: String::new(),
        age: 0,
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

    let mut p_name = &mut people.name;
    *p_name = String::from("cleiton");

    p_name = &mut input;
    *p_name = String::from("99");

    people.age = input
        .trim()
        .parse()
        .expect("erro ao converter para inteiro");

    logic(&people);
}
