use std::borrow::Cow;
use std::sync::{Mutex, Once};

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{renderer, Text};
use iced::font::Family;
use iced::widget::text::{LineHeight, Shaping};
use iced::{color, Font, Pixels};
use iced::{mouse, Point};
use iced::{Color, Element, Length, Rectangle, Size};
use serde::Deserialize;

const REGULAR_FONT_DATA: &[u8] =
    include_bytes!("../assets/font-awesome/otfs/font-awesome-6-free-regular-400.otf");

const BRANDS_FONT_DATA: &[u8] =
    include_bytes!("../assets/font-awesome/otfs/font-awesome-6-brands-regular-400.otf");

const SOLID_FONT_DATA: &[u8] =
    include_bytes!("../assets/font-awesome/otfs/font-awesome-6-free-solid-900.otf");

const REGULAR_FONT: Font = Font {
    family: Family::Name("Font Awesome 6 Free"),
    ..Font::DEFAULT
};

const SOLID_FONT: Font = Font {
    family: Family::Name("Font Awesome 6 Free"),
    weight: iced::font::Weight::Black,
    ..Font::DEFAULT
};

const BRANDS_FONT: Font = Font {
    family: Family::Name("Font Awesome 6 Brands"),
    ..Font::DEFAULT
};

static INIT: Once = Once::new();

fn load_icon_fonts<T>(renderer: &mut T)
where
    T: iced::advanced::text::Renderer,
{
    INIT.call_once(|| {
        renderer.load_font(Cow::from(REGULAR_FONT_DATA));
        renderer.load_font(Cow::from(BRANDS_FONT_DATA));
        renderer.load_font(Cow::from(SOLID_FONT_DATA));
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
    pub fn new(name: &str, font: IconFont) -> Self {
        let code = get_icon_unicode(name, &font).unwrap_or("3f".to_owned());
        let code_point = u32::from_str_radix(&code, 16).unwrap();

        let code = char::from_u32(code_point).unwrap();

        let font = match font {
            IconFont::Brands => BRANDS_FONT,
            IconFont::Default => REGULAR_FONT,
            IconFont::Solid => SOLID_FONT,
        };

        Self {
            code,
            size: 20.0,
            font,
            color: color!(0, 0, 0),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;

        self
    }

    pub fn size<T: Into<f32>>(mut self, size: T) -> Self {
        self.size = size.into();

        self
    }
}

pub fn fa_icon(name: &str) -> FaIcon {
    FaIcon::new(name, IconFont::Default)
}

pub fn fa_icon_solid(name: &str) -> FaIcon {
    FaIcon::new(name, IconFont::Solid)
}

pub fn fa_icon_brands(name: &str) -> FaIcon {
    FaIcon::new(name, IconFont::Brands)
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

#[derive(Deserialize, Clone)]
struct IconData {
    label: String,
    unicode: String,
    styles: Vec<String>,
}

static ICONS_INIT: Once = Once::new();
static ICONS_FILE_DATA: &str = include_str!("../assets/font-awesome/icons-light.json");
static mut ICONS_DATA: Option<Mutex<Vec<IconData>>> = None;

fn get_icons_data() -> &'static Mutex<Vec<IconData>> {
    unsafe {
        ICONS_INIT.call_once(|| {
            let data: Vec<IconData> =
                serde_json::from_str(ICONS_FILE_DATA).expect("Failed to parse JSON");
            ICONS_DATA = Some(Mutex::new(data));
        });

        ICONS_DATA
            .as_ref()
            .expect("ICONS_DATA should be initialized")
    }
}

fn get_icon_unicode(label: &str, font: &IconFont) -> Option<String> {
    let icons_data = get_icons_data().lock().unwrap();

    let style = match font {
        IconFont::Brands => "brands".to_owned(),
        IconFont::Default => "regular".to_owned(),
        IconFont::Solid => "solid".to_owned(),
    };

    let icon = icons_data
        .binary_search_by_key(&label.to_owned(), |item| item.label.clone())
        .ok()
        .map(|index| icons_data.get(index))??;

    if !icon.styles.contains(&style) {
        return None;
    }

    Some(icon.unicode.clone())
}
