# Too Many Linked Lists in Rust

This repository contains my personal implementations from the book **[Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html)**. The book provides a deep dive into Rust programming through the lens of building various linked list data structures. It’s a comprehensive and fun way to explore both basic and advanced Rust concepts by tackling one of the most infamous data structures in computer science.

## 📚 About the Book

The book covers the implementation of six linked lists, ranging from simple and inefficient to more advanced and practical ones. Along the way, it teaches important Rust programming concepts, including:

- **Pointer types:** `&`, `&mut`, `Box`, `Rc`, `Arc`, `*const`, `*mut`, `NonNull`.
- **Rust basics:** Ownership, borrowing, mutability, and interior mutability.
- **Core language features:** `struct`, `enum`, `fn`, `impl`, `pub`, `use`, pattern matching, generics, destructors, and more.
- **Testing and tooling:** Writing tests, using tools like `miri`, and managing Rust toolchains.
- **Unsafe Rust:** Working with raw pointers, aliasing, `UnsafeCell`, and variance.

### Lists Covered

1. **A Bad Singly-Linked Stack** ✅
2. **An Ok Singly-Linked Stack** ❌
3. **A Persistent Singly-Linked Stack** ❌
4. **A Bad But Safe Doubly-Linked Deque** ❌
5. **An Unsafe Singly-Linked Queue** ❌
6. **TODO: An Ok Unsafe Doubly-Linked Deque** ❌
7. **Bonus:** A Bunch of Silly Lists** ❌

## 🚀 Running the Code

To build and test any of the lists, ensure you have Rust installed (preferably via `rustup`). Clone this repository, navigate to the relevant list, and use `cargo` commands:

```bash
# Build the project
cargo build

# Run the tests
cargo test
