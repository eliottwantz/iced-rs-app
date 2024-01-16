use iced::{widget::text, Sandbox, Settings};

fn main() -> iced::Result {
    println!("Let's start this shit");
    Editor::run(Settings::default())
}

struct Editor;

#[derive(Debug)]
enum Message {}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        "Iced Editor".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        text("Hello iced!").into()
    }
}
