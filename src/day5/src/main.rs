fn main() {
    // Task 1. One Condition
    let my_number = 10;
    if 7 < my_number {
        println!("This number {} is larger than 7", my_number);
    } else {
        println!("This number {} is less than 7", my_number)
    }

    // Task 2. Multiple Conditions
    let test_number = 10;
    if test_number % 3 == 0 {
        println!("This value is the multiples of 3: {}", test_number);
    } else if test_number % 4 == 0 {
        println!("This value is the ultiples of 4: {}", test_number);
    } else {
        println!("This value is not division by 3 and 4")
    }

    // Task 3. Practice the `if statement` with the variable. Using if in a let Statement
    let is_vaild = true;
    let number = if is_vaild {
        1
    } else {
        -1
    };
    println!("Vaild: {}, Value: {}", is_vaild, number);    
}
