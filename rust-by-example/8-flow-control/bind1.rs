fn age() -> u32 {
    1
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("Im not born yet I guess"),
        n @ 1 ..= 12 => println!("Im a child of age {:?}", n),
        n @ 13 ..= 19 => println!("Im a teen of age {:?}", n),
        n => println!("Im an old person of age {:?}", n),
    }
}
