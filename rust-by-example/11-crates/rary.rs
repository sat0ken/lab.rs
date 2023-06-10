pub fn public_function() {
    println!("called rarys `public_function()`");
}

fn private_function() {
    println!("called rarys `private_function()`");
}

pub fn indirect_access() {
    println!("called rarys `indirect_access()`, that\n>");
    private_function();
}
