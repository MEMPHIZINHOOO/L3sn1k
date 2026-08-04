use std::path::PathBuf;
use iced::{
     Border, Color, Element, Length, Rectangle, Shadow, Size, Theme,  advanced::{ 
         Layout, Text, Widget, layout, mouse, renderer::{self, Quad}, widget::Tree
    }, alignment::Vertical, widget::text::{LineHeight, Shaping, Wrapping}
};
use proxelar_models::ProxiedRequest;


//****
// auxiliary GUI tools for utilization, keep in mind these are not the same as the "infrastructure" liable code that is currently being developed in the library of this crate
//
//
//
//


/// pub fn choose_file() opens a dialog and chooses a file, it returns an option of a pathBuf
/// be careful unwrapping this, make sure it does in fact exist beforehand, due to how iced is implemented
/// which is not idiomatic at all, we are stuck not being able to deal with unwraps utilizing propagation
/// as such we are left with this
pub fn choose_file() -> Option<PathBuf> {
    rfd::FileDialog::new().add_filter("json file", &["json"],).pick_file()
}


/// selectable proxy event widget for sending into repeater, we need this because it would be far
/// more difficult to give the information of the event to the repeater for it to be deconstructed
/// directly from a button widget
#[derive(Clone, Debug)]
pub struct ProxyEventWidget {
    proxy_event: Box<ProxiedRequest>,
    request_event_widget: String,
}

impl ProxyEventWidget {
    pub fn new(proxy_event: Box<ProxiedRequest>, request_event_widget: String) -> Self {
        Self {proxy_event: proxy_event, request_event_widget: request_event_widget}
    }

    pub fn get_proxy_event(&self) -> Box<ProxiedRequest> {
        self.proxy_event.clone()
    }
}

// rendering for proxyEventWidget
impl <Message, Renderer> Widget<Message, Theme, Renderer> for ProxyEventWidget
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
    Message: Clone,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }
    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size {
            width: 500.0,
            height: 20.0
        })
    }
    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) { 
        renderer.fill_quad(
            Quad {
                snap: false, 
                bounds: layout.bounds(),
                border: Border {
                    color: Color::from_rgb(0.6, 0.8, 1.0),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::from_rgb(0.0, 0.2, 0.4),
        );
        let bounds = layout.bounds();

        renderer.fill_text(
            Text {
                content: self.request_event_widget.clone(),
                bounds: bounds.size(),
                size: renderer.default_size(),
                line_height: LineHeight::default(),
                font: renderer.default_font(),
                align_x: iced::widget::text::Alignment::Center,
                align_y: Vertical::Center,
                shaping: Shaping::default(),
                wrapping: Wrapping::default(),
            },
            bounds.center(),
            Color::from_rgb(0.6, 0.8, 1.0),
            *viewport,
        );
        
    }
    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::Idle
        }

    
    }

    
}

impl<'a, Message, Renderer> From<ProxyEventWidget> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
    Message: 'a + Clone,
{
    fn from(widget: ProxyEventWidget) -> Self {
        Self::new(widget)
    }
}
