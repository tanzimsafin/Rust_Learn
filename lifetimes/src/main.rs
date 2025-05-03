struct Person<'a> {
    name: &'a str,
    age: u8,        
}
fn main() {
        let name="John";
        let p1 = Person {
            name: &name,
            age: 25,
        };
    println!("{} is {} years old", p1.name, p1.age);
}