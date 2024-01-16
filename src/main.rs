use iced::{
    executor,
    widget::{column, container, horizontal_space, row, text, text_editor},
    Application, Command, Length, Settings, Theme,
};

fn main() -> iced::Result {
    println!("Let's start this shit");
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Application for Editor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                content: text_editor::Content::with(include_str!("main.rs")),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Iced Editor".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.edit(action);
            }
        };

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let input = text_editor(&self.content).on_edit(Message::Edit);

        let position = {
            let (line, column) = self.content.cursor_position();

            text(format!("{}:{}", line + 1, column + 1))
        };

        let status_bar = row![horizontal_space(Length::Fill), position];

        container(column![input, status_bar.spacing(10)])
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
