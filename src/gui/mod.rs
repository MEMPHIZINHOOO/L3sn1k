// use iced::widget::column;
use iced::{
 Alignment, Event, Length, Task, event::{self}, keyboard,  widget::{button, column, radio, row
}};

mod project;
/// struct L3snikGui is responsible for the abstraction of the overall app and it's components
/// it is ran calling the start() command, and has no other public available methods, as those
/// are made for interaction with the iced crated directly
#[derive(Default)]
pub struct L3snikGui {
    project_selection :Option<u32>,
    loaded_file: Option<rfd::FileHandle>,
    loaded_file_string: String,
}

/// message enum for interactions with the widgets
#[derive(Debug, Clone)]
pub enum Message {
    ProjectSelection(u32),
    FileChosen,
}

impl L3snikGui {
    #[allow(clippy::unused_self, reason = "required by iced interface trait")]
    
    /// generates a new self object
    /// loads the default utilizing default Rust's trait derivation
    fn new() -> (Self, Task<Message>) {
        (
            Self::default(),
            Task::none(),
        )
    }
    /// updates based on received interaction with the widget
    /// takes itself as a mutable reference and a message and returns
    /// a task that can have itself a modified message for the purpose of executing tasks
    /// in broader terms it is the constant thread looking at changes done to the UI
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::ProjectSelection(value) => {self.project_selection = Some(value); Task::none() },
            Message::FileChosen => {
                if self.loaded_file.is_some() { choose_file(); self.loaded_file_string = self.loaded_file.clone().unwrap().file_name();}
                Task::none()
            }
        }
    }

    /// pretty self explanatory, I think
    fn view(&self) -> iced::Element<'_,Message> {
        row![
            column![
                radio("New Project", 1, self.project_selection, |value| Message::ProjectSelection(value)),
                radio("Load Project from file", 2, self.project_selection, |value| Message::ProjectSelection(value)),
                radio("Temporary Project", 3, self.project_selection, |value| Message::ProjectSelection(value)),
                ].spacing(50).width(Length::FillPortion(1)).align_x(Alignment::Center),

            if let  Some(value) = self.project_selection {
                if value == 2 {
                    row![
                        column![button(self.loaded_file_string.as_str()).on_press(Message::FileChosen)]                            
                    ].width(Length::FillPortion(5)).align_y(Alignment::End)                    
                }
                else {
                    row![]
                }
            }
            else {
                row![]
            }
            ].into()
    }
    /// event listener, currently only listens to the Ctrl+Q command, TODO: fix it, this shit doesn't work
    fn subscription(&self) -> iced::Subscription<Message> {
        event::listen_with( |event, _, _| match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, ..})
            if modifiers.control() && modifiers.shift() => {
                match key{
                    keyboard::Key::Character(c) => match c.as_str() {
                        "q" => std::process::exit(0),
                        _ => None,
                    }
                    _ => None,
                }
        }
        _ => None,    
        })
    }
}

impl L3snikGui {
    pub fn start() -> iced::Result {
        iced::application(L3snikGui::new, L3snikGui::update, L3snikGui::view).window(iced::window::Settings {
            size: iced::Size { width: 920.0, height: 720.0,},
            maximized: false,
            fullscreen: false,
            position:  iced::window::Position::Default,
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            closeable: true,
            minimizable: true,
            decorations: false,
            transparent: false,
            blur: false,
            level: iced::window::Level::Normal,
            icon: None,
            platform_specific: iced::window::settings::PlatformSpecific { application_id: "L3snik".to_string(), override_redirect: false },
            exit_on_close_request: true,
            })
        .title("L3snik")
        .subscription(L3snikGui::subscription)
        .run()
    }    
}

