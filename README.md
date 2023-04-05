# QuickStart
<!-- TOC -->

- [QuickStart](#quickstart)
    - [Docs](#docs)
    - [Managing Growing Projects with Packages, Crates and Modules](#managing-growing-projects-with-packages-crates-and-modules)
        - [Packages and Crates](#packages-and-crates)
        - [Defining Modules to Control Scope and Privacy](#defining-modules-to-control-scope-and-privacy)
            - [Moduels Cheat Sheet](#moduels-cheat-sheet)
    - [Iterators and Closures](#iterators-and-closures)
        - [Closures: Anonymous Functions that Capture Their Environment](#closures-anonymous-functions-that-capture-their-environment)
        - [Iterator](#iterator)
    - [Object-Oriented Programming Features of Rust](#object-oriented-programming-features-of-rust)
    - [Common Collections](#common-collections)
    - [Smart Pointers](#smart-pointers)
    - [Fearless Concurrency](#fearless-concurrency)
    - [Patterns and Matching](#patterns-and-matching)

<!-- /TOC -->
## Docs

- Rust Examples, see <https://doc.rust-lang.org/rust-by-example/index.html>

## Managing Growing Projects with Packages, Crates and Modules

A package can contain multiple binary crates and optionally one library crate.

Module System include:

- **Package**: A Cargo feature that lets you build, test and share crates.
- **Crates**: A tree of modules that produces a library or executable.
- **Modules** and **use**: Let you control the organization, scope and privacy of paths.
- **Paths**: A way of naming an item, such as a struct, function or module.

### Packages and Crates

A crate can come in one of two forms: a binary crate or a library crate.

*Binary crates* are programs you can compile to an executable that you can run.
Each must have a function called `main` that defines what happens when the
executable runs.

*Library crates* don't have a `main` function, and they don't compile to an
executable. Instead, they define functionality intended to be shared with
multiple projects.

A *package* is a bundle of one or more crates that provides a set of functionality.
A *package* can contain as many binary crates as you like, but at most only one
library crate. A package must contain at least one crate, whether that's a library
or binary crate.

If a package contains `src/main.rs` and `src/lib.rs`, it has two crates: a binary
and a library, both with the same name as the package. A package can have multiple
binary crates by placing files in the `src/bin` directory: each file will be a
separate binary crate.

### Defining Modules to Control Scope and Privacy

#### Moduels Cheat Sheet

- **Start from the crate root**: When compiling a crate, the compiler first looks in
the crate root file (usually `src/lib.rs` for a library crate or `src/main.rs`
for a binary crate) for code to compile.
- **Declaring modules**: In the crate root file, you can declare new modules; say,
you declare a "garden" module with `mod garden;`. The compiler will look for the
module's code in these places:
  - Inline, within curly brackets that replace the semicolon following `mod garden`
  - In the file `src/garden.rs`
  - In the file `src/garden/mod.rs`

- **Declaring submodules**: In any file other than the crate root, you can declare
submodules. For example, you might delcare `mod vegetables;` in `src/garden.rs`.
The compiler will look for the submodule's code within the directory named for the
parent module in these places:
  - Inline, directly following `mod vegetables`, within curly brackets instead
    of the semicolon
  - In the file `src/garden/vegetables.rs`
  - In the file `src/garden/vegetables/mod.rs`

- **Paths to code in modules**: Once a module is part of your crate, you can refer
to code in that module from anywhere else in that same crate, as long as the privacy
rules allow, using the path to the code. For example, an `Asparagus` type in the
garden vegetables moudle would be found at `crate::garden::vegetables::Asparagus`.

- **Private vs public**: Code within a module is private from its parent modules
by default. To make a module public, declare it with `pub mod` instead of `mod`.
To make items within a public module public as well, use `pub` before their declarations.

- **The `use` keyword**: Within a scope, the use keyword crates shortcuts to
items to reduce repetition of long paths. In any scope that can refer to
`crate::garden::vegetables::Asparagus`, you can create a shortcut with
`use crate::garden::vegetables::Asparagus;` and from then on you only need to
write `Asparagus` to make use of that type in the scope.

Notice that the entire module tree is rooted under the implict module named `crate`.

Rust lets you split a package into multiple crates and a crate into modules so
you can refer to items defined in one module from another module. You can do this
by specifying absolute or relative paths. These paths can be brought into scope
with a `use` statement so you can use a shorter path for multiple uses of the item
in that scope.

## Iterators and Closures

Functional programming: programming in a functional style often includes using functions
as values by passing them in arguments, returning them from other functions.

### Closures: Anonymous Functions that Capture Their Environment

Rust's closures are anonymous functions you can save in a variable or pass as arguments
to other functions.

1. `FnOnce` applies to closures that can be called once. All closures implement at least
this trait, because all closures can be called. A closure that moves captured values
out of its body will only implement `FnOnce` and none of the other `Fn` traits, because
it can only be called once.
2. `FnMut` applies to closures that don't move captured values out of their body,
but that might mutate the captured values. These closures can be called more than once.
3. `Fn` apples to closures that don't move captured values out of their body and that
don't mutate captured values, as well as closures that capture nothing from their environment.
These closures can be called more than once without mutating their environment, which is
important in cases such as calling a closure multiple times concurrently.

### Iterator

All iterators implement a trait named `Iterator` that is defined in the standard library.
The definition of the trait looks like this:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

## Object-Oriented Programming Features of Rust

OOP languages share certain common characteristics, namely objects, encapsulation,
and inheritance.

## Common Collections

Rsut's standard library includes a number of very useful data structures called
collections. Unlike the built-in array and tuple types, the data these collections
point to is stored on the heap, which means the amount of data does not need to
be known at compile time and grow or shrink as the program runs.

- A *vector* allows you to store a variable number of values next to each other.
- A *string* is a collection of characters.
- A *hash map* allows you to associate a value with a particular key. It's a
particular implementation of the more general data structure called a map.

## Smart Pointers

The concept of smart pointers isn’t unique to Rust: smart pointers originated
in C++ and exist in other languages as well. Rust has a variety of smart pointers
defined in the standard library that provide functionality beyond that provided
by references.

Rust, with its concept of ownership and borrowing, has an additional difference
between references and smart pointers: while references only borrow data, in
many cases, smart pointers own the data they point to.

Smart pointers are usually implemented using structs. Unlike an ordinary struct,
smart pointers implement the `Deref` and `Drop` traits.

The most common smart pointers in the standard library:

- `Box<T>` for allocating values on the heap
- `Rc<T>` a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces
the borrowing rules at runtime instead of compile time

## Fearless Concurrency

Handling concurrent programming safely and efficiently is another of Rust’s
major goals. Concurrent programming, where different parts of a program execute
independently, and parallel programming, where different parts of a program
execute at the same time, are becoming increasingly important as more computers
take advantage of their multiple processors. Historically, programming in these
contexts has been difficult and error prone: Rust hopes to change that.

Initially, the Rust team thought that ensuring memory safety and preventing
concurrency problems were two separate challenges to be solved with different
methods. Over time, the team discovered that the ownership and type systems are
a powerful set of tools to help manage memory safety and concurrency problems!
By leveraging ownership and type checking, many concurrency errors are
compile-time errors in Rust rather than runtime errors. Therefore, rather than
making you spend lots of time trying to reproduce the exact circumstances under
which a runtime concurrency bug occurs, incorrect code will refuse to compile
and present an error explaining the problem. As a result, you can fix your code
while you’re working on it rather than potentially after it has been shipped to
production. We’ve nicknamed this aspect of Rust fearless concurrency. Fearless
concurrency allows you to write code that is free of subtle bugs and is easy to
refactor without introducing new bugs.

## Patterns and Matching

Patterns are a special syntax in Rust for matching against the structure of types,
both complex and simple. Using patterns in conjunction with match expressions
and other constructs gives you more control over a program’s control flow. A
pattern consists of some combination of the following:

- Literals
- Destructured arrays, enums, structs, or tuples
- Variables
- Wildcards
- Placeholders
