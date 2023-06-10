fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean amoung us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}
