fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean amoung us!"),
            _ => println!("Hello {}", name),
        }
    }
}
