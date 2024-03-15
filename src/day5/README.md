## Day 5

### Task

- Declare one variable which names `my_number` with the number 10. We should check the condition of the variable with the if statement. If this variable is larger than 7, Print the `This number is larger than 7`. Or, If this variable is less than 7, Print the `This number is less than 7`.
- Create a logic to find multiples of 3 and multiples of 4. (Multiple Conditions)
- Declare the `is_valid` variable that has `true`. And Declare the `number` variable. According to the is_vaild, If is_vaild is true, number is `1`. If not, `-1`

### What I learned

- The `if statement` expects a bool type. But If there is the other type, It happens in error.
  ```
  let number = 10;
  if number {
    println!("This line is not work for the error!");
  }
  ```
- We can use the `if statement` with the variable as follows:
  ```
  let is_vaild = true;
  let number = if is_vaild {
    1
  } else {
    -1
  }
  ```

### Related Links

- https://doc.rust-lang.org/book/ch03-05-control-flow.html
