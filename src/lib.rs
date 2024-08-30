use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{renderer, Text};
use iced::widget::text::{LineHeight, Shaping};
use iced::{color, Pixels};
use iced::{mouse, Point};
use iced::{Color, Element, Length, Rectangle, Size};

const REGULAR_FONT: &[u8] =
    include_bytes!("../assets/font-awesome/otfs/font-awesome-6-free-regular-400.otf");

pub struct FaIcon {
    code: char,
    size: f32,
    color: Color,
}

impl FaIcon {
    pub fn new(_name: &str, size: f32) -> Self {
        let code_point = u32::from_str_radix("f3a3", 16).unwrap_or('\u{f005}' as u32);

        let code = char::from_u32(code_point).unwrap();

        Self {
            code,
            size,
            color: color!(255, 0, 0),
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
        renderer.load_font(REGULAR_FONT.into());

        let text = Text {
            content: &self.code.to_string(),
            bounds: layout.bounds().size(),
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: iced::alignment::Vertical::Center,
            line_height: LineHeight::Relative(self.size),
            shaping: Shaping::Basic,
            size: Pixels::from(self.size),
            font: renderer.default_font(),
        };

        renderer.fill_text(
            text,
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
