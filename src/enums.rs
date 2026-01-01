#[derive(Debug)]
enum Message {
    Resize,
}

pub fn enums() {
    println!("---{:?}", Message::Resize);
}
