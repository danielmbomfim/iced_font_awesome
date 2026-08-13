use super::*;
use iced_core::widget::text::Catalog;
use std::sync::OnceLock;

pub(crate) const REGULAR_FONT_DATA: &[u8] =
    include_bytes!("../../assets/font-awesome-6/otfs/font-awesome-6-free-regular-400.otf");

pub(crate) const BRANDS_FONT_DATA: &[u8] =
    include_bytes!("../../assets/font-awesome-6/otfs/font-awesome-6-brands-regular-400.otf");

pub(crate) const SOLID_FONT_DATA: &[u8] =
    include_bytes!("../../assets/font-awesome-6/otfs/font-awesome-6-free-solid-900.otf");

pub(crate) const REGULAR_FONT: Font = Font {
    family: Family::Name("Font Awesome 6 Free"),
    ..Font::DEFAULT
};

pub(crate) const SOLID_FONT: Font = Font {
    family: Family::Name("Font Awesome 6 Free"),
    weight: iced_core::font::Weight::Black,
    ..Font::DEFAULT
};

pub(crate) const BRANDS_FONT: Font = Font {
    family: Family::Name("Font Awesome 6 Brands"),
    ..Font::DEFAULT
};

static ICONS_FILE_DATA: &str = include_str!("../../assets/font-awesome-6/icons-light.json");
static ICONS_DATA: OnceLock<IconIndex> = OnceLock::new();

fn get_icons_data() -> &'static IconIndex {
    ICONS_DATA.get_or_init(|| IconIndex::from_json(ICONS_FILE_DATA))
}

pub(crate) fn get_icon_unicode(label: &str, font: &IconFont) -> Option<char> {
    get_icons_data().get(label, font)
}

pub struct FaIcon;

impl<'a> FaIcon {
    pub fn new<Theme>(name: &str, font: IconFont) -> Icon<'a, Theme>
    where
        Theme: Catalog,
    {
        load_icon_fonts();
        let code = get_icon_unicode(name, &font).unwrap_or('?');

        let font = match font {
            IconFont::Brands => BRANDS_FONT,
            IconFont::Default => REGULAR_FONT,
            IconFont::Solid => SOLID_FONT,
        };

        Icon {
            code,
            size: 20.0,
            font,
            color: None,
            class: Theme::default(),
        }
    }
}

pub fn fa_icon<'a, Theme: Catalog>(name: &str) -> Icon<'a, Theme> {
    FaIcon::new(name, IconFont::Default)
}

pub fn fa_icon_solid<'a, Theme: Catalog>(name: &str) -> Icon<'a, Theme> {
    FaIcon::new(name, IconFont::Solid)
}

pub fn fa_icon_brands<'a, Theme: Catalog>(name: &str) -> Icon<'a, Theme> {
    FaIcon::new(name, IconFont::Brands)
}
