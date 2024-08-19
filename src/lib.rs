mod information;
mod process;

pub use information::*;

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_all() {
        let person = Person::new("xxx".to_string());
        let mut persons = Persons::new();
        persons.push(person);
        let persons_str = persons.to_string();
        println!("{}", &persons_str);
        let return_persons: Persons = serde_json::from_str(persons_str.as_str()).unwrap();
        println!("{}", &return_persons);
        println!("{:?}", persons_str == return_persons.to_string());
    }

    #[test]
    fn test_value() {
        let json1 = json!([]);
        let json2 = json!([]);
        let persons1: Persons = serde_json::from_value(json1).unwrap();
        let persons2: Persons = serde_json::from_value(json2).unwrap();

        let mut persons = persons1.clone();
        persons += persons2;

        println!("{}", persons);
    }
}
