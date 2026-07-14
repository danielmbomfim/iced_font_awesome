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

pub use fonts::IconFont;

mod fonts {
    use std::{borrow::Cow, collections::HashMap, sync::Once};

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

    #[derive(Default)]
    struct IconIndex {
        regular: HashMap<String, char>,
        solid: HashMap<String, char>,
        brands: HashMap<String, char>,
    }

    impl IconIndex {
        fn from_json(json: &str) -> Self {
            let icons: Vec<IconData> =
                serde_json::from_str(json).expect("Failed to parse icon metadata JSON");
            let mut index = Self::default();

            for icon in icons {
                let code_point = u32::from_str_radix(&icon.unicode, 16)
                    .expect("Icon metadata should contain a hexadecimal code point");
                let code = char::from_u32(code_point)
                    .expect("Icon metadata should contain a valid Unicode code point");
                let normalized_label = normalize_label(&icon.label);

                for style in icon.styles {
                    let icons = match style.as_str() {
                        "brands" => &mut index.brands,
                        "regular" => &mut index.regular,
                        "solid" => &mut index.solid,
                        _ => continue,
                    };

                    // Canonical names are already normalized, so the common lookup
                    // path is a borrowed hash lookup with no allocation.
                    icons.entry(normalized_label.clone()).or_insert(code);
                }
            }

            index
        }

        fn get(&self, label: &str, font: &IconFont) -> Option<char> {
            let icons = match font {
                IconFont::Brands => &self.brands,
                IconFont::Default => &self.regular,
                IconFont::Solid => &self.solid,
            };

            icons.get(label).copied().or_else(|| {
                let normalized_label = normalize_label(label);
                icons.get(&normalized_label).copied()
            })
        }
    }

    fn normalize_label(label: &str) -> String {
        let mut normalized = String::with_capacity(label.len());
        let mut separator_pending = false;

        for character in label.chars() {
            if character.is_ascii_alphanumeric() {
                if separator_pending && !normalized.is_empty() {
                    normalized.push('-');
                }
                normalized.push(character.to_ascii_lowercase());
                separator_pending = false;
            } else if !normalized.is_empty() {
                separator_pending = true;
            }
        }

        normalized
    }

    #[derive(Deserialize)]
    struct IconData {
        label: String,
        unicode: String,
        styles: Vec<String>,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const ICONS: &str = r#"[
            {"label":"Circle User","unicode":"f2bd","styles":["regular","solid"]},
            {"label":"GitHub","unicode":"f09b","styles":["brands"]}
        ]"#;

        #[test]
        fn icon_index_resolves_canonical_and_flexible_labels() {
            let index = IconIndex::from_json(ICONS);

            assert_eq!(index.get("circle-user", &IconFont::Solid), Some('\u{f2bd}'));
            assert_eq!(
                index.get(" Circle__USER ", &IconFont::Default),
                Some('\u{f2bd}')
            );
            assert_eq!(index.get("github", &IconFont::Brands), Some('\u{f09b}'));
        }

        #[test]
        fn icon_index_is_scoped_by_style() {
            let index = IconIndex::from_json(ICONS);

            assert_eq!(index.get("github", &IconFont::Solid), None);
            assert_eq!(index.get("circle-user", &IconFont::Brands), None);
        }
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
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let appearance = theme.style(&self.class);

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
            self.color
                .unwrap_or(appearance.color.unwrap_or(style.text_color)),
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
