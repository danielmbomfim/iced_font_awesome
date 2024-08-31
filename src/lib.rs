use std::borrow::Cow;
use std::sync::Once;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{renderer, Text};
use iced::widget::text::{LineHeight, Shaping};
use iced::{color, Font, Pixels};
use iced::{mouse, Point};
use iced::{Color, Element, Length, Rectangle, Size};

const REGULAR_FONT: &[u8] =
    include_bytes!("../assets/font-awesome/otfs/font-awesome-6-free-regular-400.otf");

const BRANDS_FONT: &[u8] =
    include_bytes!("../assets/font-awesome/otfs/font-awesome-6-brands-regular-400.otf");

const SOLID_FONT: &[u8] =
    include_bytes!("../assets/font-awesome/otfs/font-awesome-6-free-solid-900.otf");

static INIT: Once = Once::new();

fn load_icon_fonts<T>(renderer: &mut T)
where
    T: iced::advanced::text::Renderer,
{
    INIT.call_once(|| {
        renderer.load_font(Cow::from(REGULAR_FONT));
        renderer.load_font(Cow::from(BRANDS_FONT));
        renderer.load_font(Cow::from(SOLID_FONT));
    });
}

pub enum IconFont {
    Default,
    Solid,
    Brands,
}

pub struct FaIcon {
    code: char,
    size: f32,
    color: Color,
    font: Font,
}

impl FaIcon {
    pub fn new(_name: &str, size: f32, font: IconFont) -> Self {
        let code_point = u32::from_str_radix("f3a3", 16).unwrap_or('\u{f005}' as u32);

        let code = char::from_u32(code_point).unwrap();

        let font = match font {
            IconFont::Brands => Font::with_name("Font Awesome 6 Brands"),
            IconFont::Default => Font::with_name("Font Awesome 6 Free"),
            IconFont::Solid => Font::with_name("Font Awesome 6 Free"),
        };

        Self {
            code,
            size,
            font,
            color: color!(255, 0, 0),
        }
    }
}

pub fn fa_icon(name: &str, size: f32) -> FaIcon {
    FaIcon::new(name, size, IconFont::Default)
}

pub fn fa_icon_solid(name: &str, size: f32) -> FaIcon {
    FaIcon::new(name, size, IconFont::Solid)
}

pub fn fa_icon_brands(name: &str, size: f32) -> FaIcon {
    FaIcon::new(name, size, IconFont::Brands)
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for FaIcon
where
    Renderer: iced::advanced::text::Renderer<Font = Font>,
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
        load_icon_fonts(renderer);

        let text = Text {
            content: &self.code.to_string(),
            bounds: layout.bounds().size(),
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: iced::alignment::Vertical::Center,
            line_height: LineHeight::Relative(self.size),
            shaping: Shaping::Basic,
            size: Pixels::from(self.size),
            font: self.font,
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
    Renderer: iced::advanced::text::Renderer<Font = Font>,
{
    fn from(icon: FaIcon) -> Self {
        Self::new(icon)
    }
}
