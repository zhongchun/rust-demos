fn change(s: &mut String) {
    s.push_str(", fwef");
}

fn main() {
    let mut s = String::from("heaf");

    change(&mut s);

    println!("{}", s);
}
