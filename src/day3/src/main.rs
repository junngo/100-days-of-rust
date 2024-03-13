fn main() {
    // 1. Declare the four primary scalar types and Print it
    let integer = 100;
    let decimal = 20.1;
    let boolean = true;
    let character = 'a';

    println!("Primary scalar - integer: {}, decimal: {}, boolean: {}, character: {}"
            , integer, decimal, boolean, character);

    // 2. Declare the tuple type and Print it
    let tup = (10, 200.3, true);
    println!("Declare the tuple - {} {} {}", tup.0, tup.1, tup.2);

    // Destructuring the tuple
    let (x, y, z) = tup;
    println!("Destructuring tuple - {} {} {}", x, y ,z);

    // 3. Declare the array to manage the week(sun ... sat) and print it.
    let week = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
    println!("Declare the array - {} {} {} {} {} {} {}"
            , week[0], week[1], week[2], week[3], week[4], week[5], week[6]);
}
