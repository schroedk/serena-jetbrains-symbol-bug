/// A simple struct without an impl block
pub struct Person {
    pub name: String,
    pub age: u32,
}

/// A helper function
pub fn greet(person: &Person) -> String {
    format!("Hello, {}!", person.name)
}
