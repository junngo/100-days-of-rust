## Day 1

### Task

- Create the new rust project.
- Run the project
- Print the `Hello, Rust!`

### What I learned

- Create the new rust project with the `cargo` command. We can create and run the new project as follows the command. When we use the `cargo run` command, the build job is included. Run the build job first automatically, then run the project.

```
cargo new day1
cargo run
```

- To build the project without the run. If you deploy the project to the production, Use the `--release` flag. This flag makes more lighter file size for the production but build time is long than witouht the flag. If you develop the project in local, Use the command(`cargo build`) without the flag. But If you depoly the project, Use the `--release` flag.

```
cargo build
cargo build --release
```

- If you just check the grammar and vaildation without the build and run, Use the `cargo check` command. This command

```
cargo check
```

### Related Links

- https://doc.rust-lang.org/book/ch01-00-getting-started.html
