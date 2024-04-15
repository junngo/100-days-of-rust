## Day 7

### Task

- There is no Task. Run the code(`cargo run`) and Read the code to learn ownership.

### What I learned

- Ownership Rules
  - Each value in Rust has an owner.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value wll be dropped.
- We can copy the data on the variable such as u32, f64 and bool type. There is not move action in the stack. The `x` is not freed because Value 5 saved in the stack.
  ```
  let x = 5;
  let y = x;
  ```
- If the data in on the heap such as stirng type, There is the move action when the variable is declared to initalize for the other variable. s1 is freed by move
  ```
  let s1 = String::from("Hello!");
  let s2 = s1;
  ```

### Related Links

- https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
