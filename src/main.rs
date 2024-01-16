use std::{io, path::Path, sync::Arc};

use iced::{
    executor,
    widget::{button, column, container, horizontal_space, row, text, text_editor},
    Application, Command, Length, Settings, Theme,
};

fn main() -> iced::Result {
    println!("Let's start this shit");
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
enum Message {
    FileOpened(Result<Arc<String>, Error>),
    Edit(text_editor::Action),
    OpenDialog,
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}

impl Application for Editor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                content: text_editor::Content::new(),
                error: None,
            },
            Command::perform(
                load_file(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"))),
                Message::FileOpened,
            ),
        )
    }

    fn title(&self) -> String {
        "Iced Editor".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::FileOpened(res) => {
                match res {
                    Ok(content) => self.content = text_editor::Content::with(&content),
                    Err(e) => self.error = Some(e),
                }
                Command::none()
            }
            Message::OpenDialog => Command::perform(pick_file(), Message::FileOpened),
            Message::Edit(action) => {
                self.content.edit(action);
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let controls = row![button("Open file").on_press(Message::OpenDialog)];
        let input = text_editor(&self.content).on_edit(Message::Edit);

        let position = {
            let (line, column) = self.content.cursor_position();

            text(format!("{}:{}", line + 1, column + 1))
        };

        let status_bar = row![horizontal_space(Length::Fill), position];

        container(column![controls, input, status_bar.spacing(10)])
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn pick_file() -> Result<Arc<String>, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path()).await
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|e| Error::IO(e.kind()))
}
