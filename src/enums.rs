enum TextMessage {
    Hello,
    Quit,
}

impl TextMessage {
    fn print(&self) {
        match &self {
            TextMessage::Hello => print!("Hello"),
            TextMessage::Quit => print!("Bye")
        }
    }
}

enum Message {
    Write(String)
}


impl Message {
    fn to_screen(&self) {
        match &self {
            Message::Write(m) => println!("{}", m),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::enums::{Message, TextMessage};

    
    #[test]
    fn test_enum_impl() {
        let message = TextMessage::Hello;
        message.print();
    }
    
    #[test]
    fn test_enum2() {
        let my_message = Message::Write(String::from("Hello world"));
        my_message.to_screen();
    }
}
