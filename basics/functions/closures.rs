fn main() {
    let outer_var = 42;

    let add = |i: i32| -> i32 {
        i + outer_var
    };

    let r = add(1);
    println!("r: {}", r);
}
