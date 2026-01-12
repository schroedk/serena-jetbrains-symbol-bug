/// A simple struct with an impl block
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    /// Create a new Person
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }

    /// Get the person's greeting
    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

/// A helper function
pub fn display_person(person: &Person) -> String {
    format!("{} is {} years old", person.name, person.age)
}
