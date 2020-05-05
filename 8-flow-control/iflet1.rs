fn main() {
    let number = Some(5);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    if let Some(i) = number {
        println!("Macthed {:?}", i);
    }

    if let Some(i) = number {
        println!("Macthed {:?}", i);
    } else {
        println!("Didnt match a number. Lets go with a letter!");
    }

    let i_like_letters = true;

    if let Some(i) = emotion {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didnt match a number, Lets go with a letter!");
    } else {
        println!("I dont like letter. Lets go with an emotion !");
    }

}
