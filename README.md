# Rust Learning Lab

A small, runnable Rust project that teaches core language features through code.
It uses only the standard library, so there are no third-party dependencies to
download or hide the fundamentals.

No previous Rust knowledge is required. The source comments define technical
words, explain punctuation, and describe why each example is useful. You do not
need to memorize everything on the first reading: run one lesson, connect its
printed output to the nearby code, and make one small change of your own.

## Start here

Rust was installed with the official `rustup` tool while this project was created.
Open a new terminal, then run:

```bash
cd /Users/paszti96/Documents/ChatGPT/Rust
cargo run
```

If an already-open terminal says `cargo: command not found`, load the new PATH once:

```bash
source "$HOME/.cargo/env"
```

Useful commands:

```bash
cargo run -- help         # list lessons
cargo run -- ownership    # run one lesson
cargo test                # run all automated tests
cargo fmt                 # format the source
cargo clippy --all-targets --all-features  # catch common mistakes
cargo doc --open          # build and open API documentation
```

`cargo run` compiles a debug build and runs it. The first run is slower; later
builds reuse results from the `target/` directory. For an optimized build, use
`cargo run --release`.

## How to read the comments

Rust has three comment styles in this project:

```rust
// Explains the next line or small block of code.

/// Documents the function, struct, enum, or trait immediately below it.
/// Cargo includes this text when it builds API documentation.

//! Documents the whole file or module. You will see this at the top of lessons.
```

Read the `//!` introduction first, run that lesson, and then follow the shorter
comments from top to bottom. When a comment uses an unfamiliar word, it defines
the word nearby or relates it to a familiar idea.

## Tiny syntax guide

| Syntax | Plain-language meaning |
| --- | --- |
| `let x = 5;` | Create an immutable variable named `x` |
| `let mut x = 5;` | Create a variable that may change |
| `fn name(...) -> Type` | Define a function and its returned type |
| `&value` | Borrow a value for reading |
| `&mut value` | Borrow a value for changing |
| `Type::item` | Select an item belonging to a type or module |
| `value.method()` | Call behavior using a value |
| `Some(x)` / `None` | A value exists / no value exists |
| `Ok(x)` / `Err(e)` | An operation succeeded / failed |
| `match value { ... }` | Handle every possible shape of a value |
| `<T>` | Use a generic placeholder type |
| `|x| x * 2` | A small unnamed function called a closure |
| `!` after a name | Call a macro, such as `println!` |

## Project map

| File | Topics |
| --- | --- |
| `Cargo.toml` | Package name, Rust edition, and dependencies |
| `Cargo.lock` | Exact package versions for repeatable builds |
| `.gitignore` | Generated files Git should not store |
| `src/main.rs` | Program entry point, command-line argument matching |
| `src/basics.rs` | Variables, mutability, types, functions, expressions, loops, enums |
| `src/ownership.rs` | Moves, copies, borrowing, slices, lifetimes |
| `src/oop.rs` | Structs, methods, encapsulation, traits, composition, polymorphism |
| `src/collections.rs` | `Vec`, `VecDeque`, `HashMap`, `HashSet`, `BTreeMap`, `BinaryHeap`, generics |
| `src/algorithms.rs` | Linear/binary search, insertion/merge sort, BFS, Fibonacci |
| `src/patterns.rs` | Builder, Strategy, newtype, enum-based State patterns |
| `src/errors.rs` | `Option`, `Result`, custom errors, `match`, the `?` operator |
| `src/concurrency.rs` | Scoped threads, channels, `Arc`, `Mutex`, `Send`/`Sync` concepts |
| `src/lib.rs` | Modules and library documentation |

Each lesson contains unit tests near the code it checks. Rust normally keeps unit
tests in the same file under `#[cfg(test)]`; they are omitted from regular builds.

The `src` directory contains source code. Cargo creates the `target` directory for
compiled output; you can safely delete `target` because Cargo can rebuild it.

## Rust's answer to classes and inheritance

Rust deliberately has no classes or implementation inheritance. It separates the
ideas found in class-based languages:

| Class-based idea | Rust tool |
| --- | --- |
| Object data | `struct` or `enum` |
| Constructors and methods | `impl` block |
| Interface/shared behavior | `trait` |
| Reusing fields/implementation | composition and small helper types |
| Runtime polymorphism | trait objects such as `Box<dyn Person>` |
| Compile-time polymorphism | generics such as `fn f<T: Person>(value: T)` |

This avoids fragile inheritance hierarchies. Read `src/oop.rs` while running
`cargo run -- oop` to see each replacement in action.

## The concepts that matter most

### Ownership and borrowing

Every value has one owner. When the owner leaves scope, Rust calls `drop` and
releases the resource. A move transfers ownership. `&T` temporarily borrows a
value for reading; `&mut T` borrows it for writing. At a given time, Rust allows
either many immutable references or one mutable reference. These checks prevent
data races and dangling pointers at compile time.

### Enums, matching, `Option`, and `Result`

Enums can carry data. `Option<T>` is `Some(T)` or `None`, so missing values are
explicit rather than null. `Result<T, E>` is `Ok(T)` or `Err(E)`, so recoverable
errors are ordinary values. `match` forces every possibility to be handled; `?`
is concise error propagation.

### Traits and generics

A trait declares behavior a type can implement. Generics with trait bounds give
fast static dispatch. `dyn Trait` gives runtime dispatch when different concrete
types must share one collection. Prefer static dispatch until you need the
flexibility of a trait object.

### Iterators and closures

Iterators process sequences lazily. Methods such as `map`, `filter`, `find`,
`fold`, and `collect` compose into expressive pipelines. Closures use forms such
as `|x| x * 2` and capture surrounding values when needed.

### Memory and concurrency

Rust has no garbage collector. Ownership manages stack values, heap allocations,
files, locks, and other resources through RAII: acquiring a value acquires its
resource, and dropping it releases that resource. The same type system makes many
thread-safety mistakes compile-time errors through the `Send` and `Sync` traits.

## Recommended learning order

1. Run `cargo run -- basics`, then edit a printed value.
2. Read and run `ownership`; find the commented "TRY IT" example, temporarily
   uncomment it, and study the compiler's mutable-borrow message.
3. Build a second struct implementing the `Person` trait in `oop`.
4. Add a collection transformation using `iter`, `filter`, and `collect`.
5. Add edge-case tests to an algorithm before changing its implementation.
6. Extend `PositiveNumberError` with a new error variant.
7. Run the concurrency lesson and change the number of worker threads.

The compiler is part of the learning experience. Its errors usually point to the
ownership rule or missing type information and often suggest a correct fix.

## Small exercises

- Add `Direction::Up` and update its exhaustive `match`.
- Add `Stack::len` and a test for it.
- Implement depth-first graph traversal beside breadth-first traversal.
- Add a `PickupShipping` strategy that always costs zero.
- Make `ReportBuilder` reject reports with no sections.
- Create `BookId` as another newtype and notice that it cannot be passed where a
  `UserId` is required, even though both contain `u64`.

## Where to go next

After this project, work through [The Rust Programming Language](https://doc.rust-lang.org/book/),
then solve exercises in [Rustlings](https://github.com/rust-lang/rustlings). The
standard library documentation is available at [doc.rust-lang.org/std](https://doc.rust-lang.org/std/).
