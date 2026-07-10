
// iced imports
use iced::{ Alignment, Color, Event, Length, Size, Subscription, Task, event::{self}, keyboard, widget::{Scrollable, button, column, radio, row, rule, scrollable::{Direction, Scrollbar},
    // sensor::Key, text},
    }, window};
use iced::futures::SinkExt;
use proxelar_models::ProxiedRequest;
//std imports
use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::Mutex;

use crate::gui::project::choose_file;
use crate::gui::project::ProxyEventWidget;
//tokio imports
use tokio::sync::{mpsc::{Receiver, Sender}};
//proxelar imports
use proxelar::ProxyEvent;

// http imports
use http::Method;

mod project;

static PROXY_RX: OnceLock<Mutex<Option<Receiver<ProxyEvent>>>> = OnceLock::new();

fn proxy_event_stream() -> impl iced::futures::Stream<Item = Message> {
    iced::stream::channel(100, |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
        let mut rx = PROXY_RX.get().expect("proxy_rx is not currently initialized")
            .lock().unwrap().take().expect("the proxy receiver is currently being used by another context");

        while let Some(event) = rx.recv().await {
            if output.send(Message::UpdatedProxy(event)).await.is_err() {
                break;
            }
        }
    })
}
/// struct L3snikGui is responsible for the abstraction of the overall app and it's components
/// it is ran calling the start() command, and has no other public available methods, as those
/// are made for interaction with the iced crated directly

pub struct L3snikGui {
    // init page state machine
    init_state: InitPage,
    //gui tool page state machine
    project_state: GuiToolPage,
    // project seleection structs
    project_selection :Option<u32>,
    //loaded file as a path buf so we do not lose the direct path reference
    loaded_file: Option<PathBuf>,
    //loaded file as a string
    loaded_file_string: String,

    //proxy respective vector
    proxy_vector: Vec<ProxyEvent>,
    //proxy receiver channel
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
    //updated proxy message with the newly done events
    UpdatedProxy(ProxyEvent),
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
///  match enum to string
fn match_request_values(method: &Method) -> String {
     match *method {
         Method::OPTIONS => "OPTIONS".to_string(),
         Method::GET => "GET".to_string(),
         Method::PUT => "PUT".to_string(),
         Method::PATCH => "PATCH".to_string(),
         Method::DELETE => "DELETE".to_string(),
         Method::POST => "POST".to_string(),
         Method::TRACE => "TRACE".to_string(),
         Method::CONNECT => "CONNECT".to_string(),
         Method::HEAD => "HEAD".to_string(),
         // we default to get
         _ => "GET".to_string(),
     }
} 
//formats a request into a new widget object
fn format_event(request: Box<ProxiedRequest>) -> iced::Element<'static, Message> {
            let request_value = match_request_values(request.method());
            let uri_value = request.uri();
            let request_time = request.time();
            
            let uri_path_query = match uri_value.path_and_query() {
                Some(path_value) => path_value.as_str().to_string(),
                None => "ERROR: no URL found, check for proxy logging".to_string()
            };
            
            ProxyEventWidget::new(request,format!("HTTP \t {request_value:?} \t {uri_path_query:?}  {request_time:?}")).into()    
}
///view_proxy_event formats any given proxyEvent into a Element accepted by iced
/// in this function we extract the values we want to show to the user in the proxy history
fn view_proxy_event(event: ProxyEvent) -> iced::Element<'static ,Message> {
    match event {
        //@todo: fix incomplete request complete parameters being shown
        // request complete proxyevent type
         ProxyEvent::RequestComplete {id: _ ,request, .. } => {
            format_event(request)
         },
         
         ProxyEvent::RequestIntercepted {id: _ ,request } => {
            format_event(request)
         },
         ProxyEvent::Error { message } => iced::widget::text(format!("{message}")).into(),

         ProxyEvent::WebSocketConnected { id: _, request, .. } => {
             format_event(request)
         },
        _ => iced::widget::text(format!("non implemented websocket frames and closing")).into(),
    }
}
impl L3snikGui {
    #[allow(clippy::unused_self, reason = "required by iced interface trait")]
    
    /// generates a new self object
    /// loads the default utilizing default Rust's trait derivation
    fn new() -> (Self, Task<Message>) {
        
            (
                L3snikGui {
                    init_state: InitPage::ProjectSelection,
                    project_state: GuiToolPage::Target,
                    project_selection: Some(0),
                    loaded_file: None,
                    loaded_file_string: "".to_string(),
                    proxy_vector: Vec::new(),
                },
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


            Message::UpdatedProxy(data) => {
                self.proxy_vector.push(data);
                iced::Task::none()    
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
                        
                        let proxy_formatted = column(self.proxy_vector.clone().into_iter().map(|value| view_proxy_event(value)));
                        
                        row![
                         column![
                             Scrollable::new(proxy_formatted)
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
    
    fn subscription(&self) -> iced::Subscription<Message> {
        let keyboard_sub = event::listen_with( |event, _, _| match event {
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
        });
        let proxy_sub = iced::Subscription::run(proxy_event_stream);        

        Subscription::batch(vec![keyboard_sub, proxy_sub])
    }
}

// I separated the implements so that it is easier to see the actual iced commands and the
// start function to be used at the exterior
impl L3snikGui {
    pub fn start(tx_proxy_receiver: Receiver<ProxyEvent>, tx_log_sender: Sender<anyhow::Error>) -> Result<(), ()> {
        //@TODO fix this no error handling
        let _ = PROXY_RX.set(Mutex::new(Some(tx_proxy_receiver)));        
        match iced::application(L3snikGui::new,
         L3snikGui::update,
         L3snikGui::view
         ).window(iced::window::Settings {
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
        .run() {
            Ok(_) => {},
            Err(error) => {tx_log_sender.try_send(anyhow::Error::new(error)).expect("log sender isn't working, shutting down now");},
        };
        Ok(())
    }    
}

