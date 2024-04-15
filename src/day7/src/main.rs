fn main() {
    let x = 5;
    let y = x;
    println!("x: {x}, y: {y}");

    // 1. Move Concept in rust
    let s1 = String::from("Hello!");
    let s2 = s1;
    println!("s1: (s1 value is freed after moving into s2), s2: {s2}");
    // If you call the `s1` variable, We meet the error. The `s1` is moved into the `s2`
    // println!("s1: {s1}, s2:{s2}");

    // 2. Using the clone
    let s1 = s2.clone();
    println!("s1: {s1}, s2: {s2}");

    // 3. Ownership and Fucntions
    let s3 = String::from("Ownership test");
    takes_ownership(s3);
    // S3 is freed by calling the takes_ownership. That happens the error without the comment below the code.
    // println!("{s3}");

    makes_copy(x);
    println!("x: {x} after calling the makes_copy");

    // 4. Ownership and Function return value
    let s4 = gives_ownership();
    println!("s4: {s4}");

    let s5 = String::from("Function return value test");
    let s6 = takes_and_gives_back(s5);

    // println!("s5: {s5}");
    println!("s6: {s6}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}
fn takes_and_gives_back(some_string: String) -> String {
    some_string
}