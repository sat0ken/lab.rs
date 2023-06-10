fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("Im a function!");
}

fn main() {
    let closure = || println!("Im a closure!");

    call_me(closure);
    call_me(function);
}
