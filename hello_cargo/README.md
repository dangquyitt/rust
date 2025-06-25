# Cargo

## Let’s recap what we’ve learned so far about Cargo:

1. We can create a project using `cargo new`.
2. We can build a project using `cargo build`.
3. We can build and run a project in one step using `cargo run`.
4. We can build a project without producing a binary to check for errors using `cargo check`.
   Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
5. `cargo build --release` to compile it with optimizations. This command will create an executable in target/release instead of target/debug.
