use iced::{widget::text, Element, Sandbox, Settings};

fn main() -> iced::Result {
    Konw::run(Settings::default())
}

struct Konw;

#[derive(Debug)]
enum Message {}

impl Sandbox for Konw {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Konwerter Systemów Liczbowych")
    }

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view(&self) -> Element<'_, Self::Message> {
        text("Hello").into()
    }
}
