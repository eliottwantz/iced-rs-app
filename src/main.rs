use std::{
    io,
    path::{Path, PathBuf},
    sync::Arc,
};

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
    path: Option<PathBuf>,
    content: text_editor::Content,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
enum Message {
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
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
                path: None,
                content: text_editor::Content::new(),
                error: None,
            },
            Command::perform(
                load_file(string_to_path(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))),
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
                    Ok((path, content)) => {
                        self.path = Some(path);
                        self.content = text_editor::Content::with(&content)
                    }
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

        let file_path = match self.path.as_deref().and_then(Path::to_str) {
            Some(path) => text(path).size(14),
            None => text(""),
        };

        let position = {
            let (line, column) = self.content.cursor_position();

            text(format!("{}:{}", line + 1, column + 1))
        };

        let status_bar = row![file_path, horizontal_space(Length::Fill), position];

        container(column![controls, input, status_bar.spacing(10)])
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn string_to_path(s: String) -> PathBuf {
    PathBuf::from(s)
}

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(handle.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let content = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|e| Error::IO(e.kind()))?;

    Ok((path, content))
}
