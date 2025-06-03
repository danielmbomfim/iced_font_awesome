use std::borrow::Cow;
use std::sync::{Mutex, Once};

use iced::advanced::graphics::text::font_system;
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{renderer, Text};
use iced::font::Family;
use iced::widget::text::{Catalog, LineHeight, Shaping, Style, StyleFn};
use iced::{mouse, Point};
use iced::{Color, Element, Length, Rectangle, Size};
use iced::{Font, Pixels};
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

fn load_icon_fonts() {
    INIT.call_once(|| {
        let mut font_system = font_system().write().unwrap();

        font_system.load_font(Cow::from(REGULAR_FONT_DATA));
        font_system.load_font(Cow::from(BRANDS_FONT_DATA));
        font_system.load_font(Cow::from(SOLID_FONT_DATA));
    });
}

pub enum IconFont {
    Default,
    Solid,
    Brands,
}

pub struct FaIcon<'a, Theme: Catalog> {
    code: char,
    size: f32,
    color: Option<Color>,
    font: Font,
    class: Theme::Class<'a>,
}

impl<'a, Theme: Catalog> FaIcon<'a, Theme> {
    pub fn new(name: &str, font: IconFont) -> Self {
        load_icon_fonts();
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
            color: None,
            class: Theme::default(),
        }
    }

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

pub fn fa_icon<'a, Theme: Catalog>(name: &str) -> FaIcon<'a, Theme> {
    FaIcon::new(name, IconFont::Default)
}

pub fn fa_icon_solid<'a, Theme: Catalog>(name: &str) -> FaIcon<'a, Theme> {
    FaIcon::new(name, IconFont::Solid)
}

pub fn fa_icon_brands<'a, Theme: Catalog>(name: &str) -> FaIcon<'a, Theme> {
    FaIcon::new(name, IconFont::Brands)
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for FaIcon<'a, Theme>
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
            align_x: iced::alignment::Horizontal::Center.into(),
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

impl<'a, Message, Theme, Renderer> From<FaIcon<'a, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::text::Renderer<Font = Font>,
    Theme: Catalog + 'a,
{
    fn from(icon: FaIcon<'a, Theme>) -> Self {
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

#[allow(static_mut_refs)]
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
