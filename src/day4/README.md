## Day 4

### Task

- Create the `print_hello_rust` function to print `Hello, Rust:)` and call it in the main.
- Create the `print_number_param` function that has a `i32` parameter. And then print the number such as `My Number: 30`.
- Create the `print_two_number` function that has a `i32` parameter. Write the code with the `Statements` and `Expressions` to print the two number.
- Create the `get_11_number` fucntion that return the `11` value.

### What I learned

- Argument and Parameter concepts are not the same.
  - Argument: The value to hand into function.
  - Parameter: The value that function has.
- Statements and Expressions concepts.
  - Statements: Perform some action but do not return a value.
    ```
    let i = 10;
    ```
  - Expressions: Return the result value. Block `{}` is expressions too.
    ```
    let j = {
      let a = 11;
      a + 10
    };
    ```
    There is not a semicolon at the end. It means, return the value.
- Functions with the return value are unnecessary to use the `return` keyword in the rust.

### Related Links

- https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
