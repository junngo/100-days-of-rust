## Day 8

### Task

- Make the function to caculate the string lenght. function name is the `caculate_length`. The function has the `reference param` and `usize return` value. And call the function.
- Make the function to change the String. The function name is the `change_string_value`. The function concat the `, world` with the parameter. (mut conncept)

### What I learned

- If we just give out the string variable into the function, the variable is freed. That is the ownership concept. When We want to give out the value for referring the value, we can use the reference concept(&). We can't change the reference value, Just can read it.
- If we should change the value of prameter, We should use the mut keyword.
- Dangling Concept
  ```
  fn main() {
    let reference_to_nothing = dangle();
  }
  fn dangle() -> &String {
    let s = String::from("Hello!!");
    &s
  }
  fn no_dangle() -> String {
    let s = String::from("Hello!!!");
    s
  }
  ```

### Related Links

- https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
