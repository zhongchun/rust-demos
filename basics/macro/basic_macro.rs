// Why are macros usefull?
// 1. Don't repeat yourself. There are many cases where you may need similar
// functionality in multiple places but with different types. Often, writing
// a macro is a usefull way to avoid repeating code.
// 2. Domain-specific languages. Macros allow you to define special syntax
// for a specific purpose.
// 3. Variadic interfaces. Sometimes you want to define an interface that
// takes a variable number of argumetns. An example `println!` which could
// take any number of arguments, depending on the format string.
// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

fn main() {
    // This call will expand into `println!("Hello");`
    say_hello!()
}
