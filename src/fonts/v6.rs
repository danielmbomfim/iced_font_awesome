use super::*;

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
    weight: iced::font::Weight::Black,
    ..Font::DEFAULT
};

pub(crate) const BRANDS_FONT: Font = Font {
    family: Family::Name("Font Awesome 6 Brands"),
    ..Font::DEFAULT
};

static ICONS_INIT: Once = Once::new();
static ICONS_FILE_DATA: &str = include_str!("../../assets/font-awesome-6/icons-light.json");
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

pub(crate) fn get_icon_unicode(label: &str, font: &IconFont) -> Option<String> {
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

pub struct FaIcon;

impl<'a> FaIcon {
    pub fn new<Theme>(name: &str, font: IconFont) -> Icon<'a, Theme>
    where
        Theme: Catalog,
    {
        load_icon_fonts();
        let code = get_icon_unicode(name, &font).unwrap_or("3f".to_owned());
        let code_point = u32::from_str_radix(&code, 16).unwrap();

        let code = char::from_u32(code_point).unwrap();

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
