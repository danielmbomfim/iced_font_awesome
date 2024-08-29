
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{renderer, Text};
use iced::widget::text::{LineHeight, Shaping};
use iced::{color, Pixels};
use iced::{mouse, Point};
use iced::{Color, Element, Length, Rectangle, Size};

pub struct FaIcon {
    name: String,
    size: f32,
    color: Color,
}

impl FaIcon {
    pub fn new(name: &str, size: f32) -> Self {
        Self {
            name: name.to_owned(),
            size,
            color: color!(0, 0, 0),
        }
    }
}

pub fn fa_icon(name: &str, size: f32) -> FaIcon {
    FaIcon::new(name, size)
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for FaIcon
where
    Renderer: iced::advanced::text::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.size, self.size))
    }

    fn draw(
        &self,
        _state: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        renderer.fill_text(
            Text {
                content: "F",
                bounds: layout.bounds().size(),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                line_height: LineHeight::Relative(self.size),
                shaping: Shaping::Basic,
                size: Pixels::from(self.size),
                font: renderer.default_font(),
            },
            Point::new(layout.bounds().center_x(), layout.bounds().center_y()),
            self.color,
            *viewport,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<FaIcon> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::text::Renderer,
{
    fn from(icon: FaIcon) -> Self {
        Self::new(icon)
    }
}
