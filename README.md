# QuickStart

## Docs
- Examples: https://doc.rust-lang.org/rust-by-example/index.html

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

* **Start from the crate root**: When compiling a crate, the compiler first looks in
the crate root file (usually `src/lib.rs` for a library crate or `src/main.rs`
for a binary crate) for code to compile.
* **Declaring modules**: In the crate root file, you can declare new modules; say,
you declare a "garden" module with `mod garden;`. The compiler will look for the
module's code in these places:
    - Inline, within curly brackets that replace the semicolon following `mod garden`
    - In the file `src/garden.rs`
    - In the file `src/garden/mod.rs`
* **Declaring submodules**: In any file other than the crate root, you can declare
submodules. For example, you might delcare `mod vegetables;` in `src/garden.rs`.
The compiler will look for the submodule's code within the directory named for the
parent module in these places:
    - Inline, directly following `mod vegetables`, within curly brackets instead
    of the semicolon
    - In the file `src/garden/vegetables.rs`
    - In the file `src/garden/vegetables/mod.rs`
* **Paths to code in modules**: Once a module is part of your crate, you can refer
to code in that module from anywhere else in that same crate, as long as the privacy
rules allow, using the path to the code. For example, an `Asparagus` type in the
garden vegetables moudle would be found at `crate::garden::vegetables::Asparagus`.
* **Private vs public**: Code within a module is private from its parent modules
by default. To make a module public, declare it with `pub mod` instead of `mod`.
To make items within a public module public as well, use `pub` before their declarations.
* **The `use` keyword**: Within a scope, the use keyword crates shortcuts to
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
