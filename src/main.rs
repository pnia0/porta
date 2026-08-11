mod core;

use std::sync::LazyLock;
use std::path::PathBuf;
use std::env;
use iced::{Center, Color, Element, Event, Task as Command, event};
use iced::widget::operation::focus;
use iced::widget::{Column, button, column, text_input};
use iced_layershell::Settings;
use iced_layershell::reexport::{ Anchor, KeyboardInteractivity };
use iced_layershell::build_pattern::application;
use iced_layershell::settings::{LayerShellSettings, StartMode};
use iced_layershell::to_layer_message;

static CONFIG_PATH: &str = "porta/config.toml";
static INPUT_ID: LazyLock<iced::widget::Id> = LazyLock::new(iced::widget::Id::unique);

pub fn main() -> Result<(), iced_layershell::Error> {
    let binded_output_name = std::env::args().nth(1);
    let start_mode = match binded_output_name {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };

    application(Launcher::new, Launcher::namespace, Launcher::update, Launcher::view) .style(style)
        .subscription(Launcher::subscription)
        .settings(Settings {
            layer_settings: LayerShellSettings {
                size: Some((250, 400)),
                exclusive_zone: 400,
                anchor: Anchor::Left | Anchor::Right,
                keyboard_interactivity: KeyboardInteractivity::Exclusive,
                start_mode,
                ..Default::default()
            },
            ..Default::default()
        })
        .run()?;
    Ok(())
}

struct Launcher {
    command: String,
    aliases: Option<core::Aliases>,
    pending_launch: Option<String>
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    SearchEditChanged(String),
    Launch,
    AliasesLoaded(Result<core::Aliases, String>),
    IcedEvent(Event),
}

impl Launcher {
    fn new() -> (Self, Command<Message>) {
        let focus_task = focus(INPUT_ID.clone());

        let config_task = Command::perform(
            async move { core::Aliases::load_from_file(get_config_path()) },
            Message::AliasesLoaded
        );
        let tasks = Command::batch(vec![focus_task, config_task]);
        (
            Self {
                command: String::new(),
                aliases: None,
                pending_launch: None,
            },
            tasks
        )
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        use iced_runtime::{ Action, keyboard };
        use keyboard::key::Named;
        match message {
            Message::IcedEvent(event) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event {
                    if key == keyboard::Key::Named(Named::Enter) {
                        return Command::done(Message::Launch)
                    }
                }
                Command::none()
            }
            Message::SearchEditChanged(edit) => {
                self.command = edit;
                Command::none()
            }
            Message::Launch => {
                if let Some(ref aliases) = self.aliases {
                    match core::launch(&self.command, aliases) {
                        Ok(_) => {
                            return iced_runtime::task::effect(Action::Exit)
                        }
                        Err(_) => {
                            return Command::none()
                        }
                    }
                }
                self.pending_launch = Some(self.command.clone());
                Command::none()
            }
            Message::AliasesLoaded(result) => {
                match result {
                    Ok(aliases) => {
                        if let Some(ref command) = self.pending_launch {
                            match core::launch(command, &aliases) {
                                Ok(_) => {
                                    return iced_runtime::task::effect(Action::Exit)
                                }
                                Err(_) => {
                                    return Command::none()
                                }
                            }
                        }
                        self.aliases = Some(aliases);
                        Command::none()
                    }
                    Err(_) => {
                        Command::none()
                    }
                }
            }
            _ => unreachable!()
        }
    }

    fn view(&self) -> Column<'_, Message> {
        let command_input: Element<Message> = text_input("input command", &self.command)
            .on_input(Message::SearchEditChanged)
            .id(INPUT_ID.clone())
            .into();
        column![
            command_input
        ]
        .padding(20)
        .align_x(Center)
    }

    fn namespace() -> String {
        String::from("porta")
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        event::listen().map(Message::IcedEvent)
    }
}

fn style(_counter: &Launcher, theme: &iced::Theme) -> iced::theme::Style {
    iced::theme::Style {
        background_color: Color::TRANSPARENT,
        text_color: theme.palette().text,
    }
}

fn get_config_path() -> PathBuf {
    if let Ok(path) = env::var("PORTA_CONFIG") {
        return PathBuf::from(path)
    }
    dirs::config_dir()
        .unwrap_or_default()
        .join(CONFIG_PATH)
}
