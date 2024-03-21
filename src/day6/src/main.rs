fn main() {
    // 1. Initalize the variable from the loop
    let mut count = 0;

    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("result: {result}");

    // 2. practice while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    // 3. Loop over the elements of collection
    let a = [10, 20, 30, 40, 50];
    for i in a {
        println!("value of element: {i}")
    }

    // 4. practice the for loop
    for i in 1..4 {
        println!("value of for loop: {i}")
    }

    // Task1. Print the number between 1 and 10 by for loop
    for i in 1..11 {
        println!("number: {i}")
    }

    // Task2. Print the four thimes table
    for i in 1..10 {
        println!("4 X {i} = {}", i * 4)
    }

    // Task3. Multiply each element of array by 2
    let my_array = [10, 20, 30, 40, 50];
    for i in my_array {
        println!("i: {}", i * 2)
    }
}
