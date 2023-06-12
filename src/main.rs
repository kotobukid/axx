fn main() {
    println!("Hello, world!");
    println!("{}", greet("Taro".into()));
}

fn greet (name: String) -> String {
    format!("hello {}", &name)
}