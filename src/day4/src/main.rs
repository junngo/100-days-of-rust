fn main() {
    print_hello_rust();
    print_number_param(30);
    print_two_number(10);

    let x = get_11_number();
    println!("x: {}", x);
}

fn print_hello_rust() {
    println!("Hello, Rust");
}
fn print_number_param(num: i32) {
    println!("My Number: {}", num);
}
fn print_two_number(num: i32) {
    let x = num;
    let y = {
        let x = 3;
        x + 5
    };
    println!("x: {}, y: {}", x, y);
}
fn get_11_number() -> i32 {
    11
}
