pub mod protobuf {
    pub mod items {
        include!(concat!(env!("OUT_DIR"), "/protobuf.items.rs"));
    }
}

use protobuf::items;

pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    shirt.set_size(items::shirt::Size::Large);
    shirt
}

fn main() {
    let shirt = create_large_shirt("blue".to_string());
    println!("shirt: {:?}", shirt);
}
