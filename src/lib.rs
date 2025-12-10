use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{renderer, Text};
use iced::widget::text::{Catalog, LineHeight, Shaping, Style, StyleFn};
use iced::{mouse, Point};
use iced::{Color, Element, Length, Rectangle, Size};
use iced::{Font, Pixels};

#[cfg(feature = "v6")]
pub use crate::fonts::v6;

#[cfg(feature = "v7")]
pub use crate::fonts::v7::{fa_icon, fa_icon_brands, fa_icon_solid, FaIcon};

mod fonts {
    use std::{
        borrow::Cow,
        sync::{Mutex, Once},
    };

    #[cfg(feature = "v6")]
    pub mod v6;
    #[cfg(feature = "v7")]
    pub mod v7;

    use iced::widget::text::Catalog;
    use iced::{advanced::graphics::text::font_system, font::Family, Font};
    use serde::Deserialize;

    use crate::Icon;

    static INIT: Once = Once::new();

    pub fn load_icon_fonts() {
        INIT.call_once(|| {
            let mut font_system = font_system().write().unwrap();

            #[cfg(feature = "v6")]
            {
                font_system.load_font(Cow::from(v6::REGULAR_FONT_DATA));
                font_system.load_font(Cow::from(v6::BRANDS_FONT_DATA));
                font_system.load_font(Cow::from(v6::SOLID_FONT_DATA));
            }
            #[cfg(feature = "v7")]
            {
                font_system.load_font(Cow::from(v7::REGULAR_FONT_DATA));
                font_system.load_font(Cow::from(v7::BRANDS_FONT_DATA));
                font_system.load_font(Cow::from(v7::SOLID_FONT_DATA));
            }
        });
    }

    pub enum IconFont {
        Default,
        Solid,
        Brands,
    }

    #[derive(Deserialize, Clone)]
    struct IconData {
        label: String,
        unicode: String,
        styles: Vec<String>,
    }
}

pub struct Icon<'a, Theme: Catalog> {
    code: char,
    size: f32,
    color: Option<Color>,
    font: Font,
    class: Theme::Class<'a>,
}

impl<'a, Theme: Catalog> Icon<'a, Theme> {
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);

        self
    }

    pub fn size<T: Into<f32>>(mut self, size: T) -> Self {
        self.size = size.into();

        self
    }

    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Icon<'a, Theme>
where
    Renderer: iced::advanced::text::Renderer<Font = Font>,
    Theme: Catalog,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
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
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let style = theme.style(&self.class);

        let text = Text {
            content: self.code.to_string(),
            bounds: layout.bounds().size(),
            align_x: widget::text::Alignment::Center,
            align_y: iced::alignment::Vertical::Center,
            line_height: LineHeight::Relative(self.size),
            shaping: Shaping::Basic,
            size: Pixels::from(self.size),
            font: self.font,
            wrapping: widget::text::Wrapping::None,
        };

        renderer.fill_text(
            text,
            Point::new(layout.bounds().center_x(), layout.bounds().center_y()),
            self.color.unwrap_or(style.color.unwrap_or(Color::WHITE)),
            *viewport,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Icon<'a, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::text::Renderer<Font = Font>,
    Theme: Catalog + 'a,
{
    fn from(icon: Icon<'a, Theme>) -> Self {
        Self::new(icon)
    }
}
