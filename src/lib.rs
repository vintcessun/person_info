mod person;

pub use person::{Person, Persons, Relation};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let person = Person::new("xxx".to_string());
        println!("{}",person);
    }
}
