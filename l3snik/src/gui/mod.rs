// use iced::widget::column;
use iced::{ window, Size,
 Alignment, Event, Length, Task, event::{self}, keyboard,  widget::{button, column, radio, row, rule, text, Scrollable, scrollable::{Direction, Scrollbar}}};
use std::{path::PathBuf};
use crate::gui::project::choose_file;

use std::error::Error;


mod project;
/// struct L3snikGui is responsible for the abstraction of the overall app and it's components
/// it is ran calling the start() command, and has no other public available methods, as those
/// are made for interaction with the iced crated directly
#[derive(Default)]


pub struct L3snikGui {
    init_state: InitPage,
    project_state: GuiToolPage,
    // project seleection structs
    project_selection :Option<u32>,
    //loaded file as a path buf so we do not lose the direct path reference
    loaded_file: Option<PathBuf>,
    //loaded file as a string
    loaded_file_string: String,
}

/// message enum for interactions with the widgets
#[derive(Debug, Clone)]
pub enum Message {
    // project selection respective to a button and the respective choice
    ProjectSelection(u32),
    //the file chosen
    FileChosen,
    // continuining onto the actual project
    Continue,

    /// DECLARATION OF MESSAGE TYPES IN THE Project type

    // proxy page
    Proxy,
    // repeater page
    Repeater,
    // target page
    Target,
}

// two pages, select the project options and the actual project
#[derive(Default, PartialEq)]
pub enum InitPage {
    #[default]
    ProjectSelection,
    Project,
}

#[derive(Default)]
pub enum GuiToolPage {
    Repeater,
    #[default]
    Target,
    Proxy,
    
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
    fn resize_window(width: f32, height: f32) -> impl Fn(window::Id) -> Task<Message> {
        move |window_id: window::Id| {
            window::resize::<Message>(window_id, Size::new(width, height))
        }
    }
    /// updates based on received interaction with the widget
    /// takes itself as a mutable reference and a message and returns
    /// a task that can have itself a modified message for the purpose of executing tasks
    /// in broader terms it is the constant thread looking at changes done to the UI
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            // matches the selected value of the projectSelection
            Message::ProjectSelection(value) => {self.project_selection = Some(value); Task::none() },
            // gets the chosen file path into the screen
            Message::FileChosen => {

                self.loaded_file = choose_file();
        
                if self.loaded_file.is_some()
                {
                    //we need to rewrite these unwraps so they are handled, eventually.
                    self.loaded_file_string = self.loaded_file.clone().unwrap().to_str().unwrap().to_string();
                }
                            
                else {
                    self.loaded_file_string = "Choose a File".to_string();
                }
                           Task::none()
            }
            Message::Continue => {
                self.init_state = InitPage::Project;
                return window::latest().and_then(Self::resize_window(1600.0, 1080.0))
                
            }
            
            Message::Proxy => {
                self.project_state = GuiToolPage::Proxy;
                Task::none()    
            }
            
            Message::Repeater => {
                self.project_state = GuiToolPage::Repeater;
                Task::none()    
            }
            
            Message::Target => {
                self.project_state = GuiToolPage::Target;
                Task::none()    
            }
            
        }
    }

    /// pretty self explanatory, I think, it is divided into the project selection type and the project itself
    /// this needs to be refactored into smaller portions of code
    fn view(&self) -> iced::Element<'_,Message> {
        // project selection type case
        if self.init_state == InitPage::ProjectSelection {
            row![
                column![
                    radio("New Project", 1, self.project_selection, |value| Message::ProjectSelection(value)),
                    radio("Load Project from file", 2, self.project_selection, |value| Message::ProjectSelection(value)),
                    radio("Temporary Project", 3, self.project_selection, |value| Message::ProjectSelection(value)),
                    ].spacing(50).width(Length::FillPortion(1)).align_x(Alignment::Center),
                    column![
                        button("Continue").on_press(Message::Continue) 
                    ],
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

        else {
            column![
                // the navigation menu
                row![
                    button("Proxy").on_press(Message::Proxy),
                    button("Repeater").on_press(Message::Repeater),
                    button("Target").on_press(Message::Target),
                ].spacing(100),
                rule::horizontal(10),

                // match for the guitoolpages
                match self.project_state {

                    GuiToolPage::Proxy => {
                        let test_values = column((0..10).map( |_| text("test values").into()));
                        row![
                            column![
                                Scrollable::new(test_values)
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .direction(Direction::Vertical(Scrollbar::new())),
                            ].width(Length::FillPortion(2)),
                            column![].width(Length::FillPortion(3)),
                        ]
                    }

                    GuiToolPage::Repeater => {
                        row![]
                    }

                    GuiToolPage::Target => {
                        row![]
                    }
                }
            ].into()
        }    
    }
    /// event listener, currently only listens to the Ctrl+Q command, TODO: fix it, this shit doesn't work
    fn subscription(&self) -> iced::Subscription<Message> {
        event::listen_with( |event, _, _| match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, ..})
            if modifiers.control() => {
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
    pub fn start() -> Result<(), Box<dyn Error + 'static>> {
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
            decorations: true,
            transparent: false,
            blur: false,
            level: iced::window::Level::Normal,
            icon: None,
            platform_specific: iced::window::settings::PlatformSpecific { application_id: "L3snik".to_string(), override_redirect: false },
            exit_on_close_request: true,
            })
        .title("L3snik")
        .subscription(L3snikGui::subscription)
        .run()?;
        Ok(())
    }    
}

