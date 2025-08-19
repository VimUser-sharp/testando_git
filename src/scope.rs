use crate::Person;

pub fn logic(entity: &Person) {
    let variable: String = String::from("hello");
    //let other_variable: &str = "francisco";

    println!("{variable} {}\nidade: {}", entity.name, entity.age);
}
