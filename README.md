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
    - [Advanced Features](#advanced-features)
        - [Macros](#macros)
            - [The Difference Between Macros and Functions](#the-difference-between-macros-and-functions)
            - [Declarative Macros with macro_rules! for General Metaprogramming](#declarative-macros-with-macro_rules-for-general-metaprogramming)
            - [Procedural Macros for Generating Code from Attributes](#procedural-macros-for-generating-code-from-attributes)
                - [Custom derive macros](#custom-derive-macros)
                - [Attribute-like macros](#attribute-like-macros)
                - [Function-like macros](#function-like-macros)

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

## Advanced Features

- Unsafe Rust: how to opt out of some of Rust’s guarantees and take responsibility
for manually upholding those guarantees
- Advanced traits: associated types, default type parameters, fully qualified
syntax, supertraits, and the newtype pattern in relation to traits
- Advanced types: more about the newtype pattern, type aliases, the never type,
and dynamically sized types
- Advanced functions and closures: function pointers and returning closures
- Macros: ways to define code that defines more code at compile time

### Macros

The term macro refers to a family of features in Rust: *declarative macros* with
`macro_rules!` and three kinds of *procedural macros*:

- Custom `#[derive]` macros that specify code added with the `derive` attribute
used on structs and enums
- Attributed-like macros that define custom attributes usable on an item
- Function-like macros that look like function calls but operate on the token
specified as their argument

#### The Difference Between Macros and Functions

Fundamentally, macros are a way of writing code that writes other code, which is
known as *metaprogramming*.

- Metaprogramming is usefurl for reducing the amount of code you have to write
and maintain, which is also one of the roles of functions. However, macros have
some additional powers than functions don't.
  - A function signature must delcare the number and type of parameters the
  function has. Macros, on the other hand, can take a variable number of parameters.
  Also, macros are expandted before the compiler interpres the meaning of the code,
  so a macro can, for example, implement a trait on a given type. A function can't,
  because it gets called at runtime and a trait needs to be implemented at compile
  time.
  - The downside to implementing a macro instead of a function is that macro
  definitions are more complex than function definitions because you're writing
  Rust code that writes Rust code. Due to this indirection, macro definitions are
  generally more difficult to read, understand, and maintain than function definitions.
- You must define macros or bring them into scope before you call them in a file,
as opposed to functions you can define anywhere and call anywhere.

#### Declarative Macros with `macro_rules!` for General Metaprogramming

The most widely used form of macros in Rust is the *declarative marcro*. These
are also sometimes referred to as "macros by example", "`macro_rules!` macros"
or just plain "macros". At their core, declarative macros allow you to write
something similar to a Rust `match` expression, like `println!`, `vec!`.

#### Procedural Macros for Generating Code from Attributes

The second form of macros is the *procedural macro*, which acts more like a
function (and is a type of procedure). Procedural macros accept some code as
an input, operate on that code, and produce some code as an output rather than
matching agginst patterns and replcaing the code with other code as declarative
maacros do. The three kinds of procedural macros are *custom derive*,
*attribute-like*, and *function like*, and all work in a similar fashion.

##### Custom derive macros

`derive` only works for structs and enums.

##### Attribute-like macros

Attribute-like macros are similar to custom derive macros, but instead of
generating code for the `derive` attribute, they allow youo to create new
attributes. They are more flexible: `derive` only works for structs and enums;
attributes can be applied to other item as well, such as functions.

Attribute-like macros work the same way as custom derive macros: you create a
crate with the `proc-macro` crate type and implement a function that generates
the code you want!

##### Function-like macros

Function-like macros define macros that look like function calls. Similarly to
`macro_rules!` macros, they're more flexible than functions; for example, they
can take an unknown number of arguments. However, `macro_rules!` macros can be
defined only using the match-like syntax. Function-like macros take a `TokenStream`
parameter and their definition manipulates that `TokenStream` using Rust code as
the other two types of procedural macros. An example of a function-like macro is
an sql! macro that might be called like so:

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

This macro would parse the SQL statement inside it and check that it’s
syntactically correct, which is much more complex processing than a
`macro_rules!` macro can do. The sql! macro would be defined like this:

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

This definition is similar to the custom derive macro’s signature: we receive
the tokens that are inside the parentheses and return the code we wanted to generate.
