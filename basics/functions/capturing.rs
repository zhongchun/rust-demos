fn main() {
    use std::mem;

    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    let _count_reborrowed = &mut count;

    let movable = Box::new(3);
    let consume = || {
       println!("`movable`: {:?}", movable);
       mem::drop(movable);
    };

    consume();
}
