/*
fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess={}", guess);

    let y = {
        let x = 3;
        x + 1
    };
    println!("y={}", y);
}
*/

/*
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/

fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    let x = 69420;
    println!("Base 10:          {}", x);
    println!("Base 2 :          {:b}", x);
    println!("Base 8 :          {:o}", x);
    println!("Base 16:          {:x}", x);
    println!("Base 16:          {:X}", x);
    println!("{number:>5}", number=1);
    println!("{number:0<5}", number=1);
    println!("{number:0>width$}", number=1, width=5);
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
