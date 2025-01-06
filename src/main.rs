fn main() {
    println!("Hello, world!");
    let c = add_two_strings(String::from("Hello"), String::from("world") );
    println!("{}", c)
}

fn add_two_strings(a: String, b: String) -> String {
    a + &", " + &b + &"!"
}
