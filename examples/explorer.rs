use std::sync::{Mutex, Once};

use iced::alignment::Horizontal;
use iced::widget::scrollable::{Direction, Properties};
use iced::widget::{button, column, row, text, text_input, Scrollable};
use iced::{color, Application, Command, Element, Length, Theme};
use iced_font_awesome::{fa_icon, fa_icon_brands, fa_icon_solid};
use serde::Deserialize;

pub fn main() -> iced::Result {
    let mut settings = iced::Settings::default();
    settings.window.size = iced::Size::new(800.0, 300.0);
    Explorer::run(settings)
}

#[derive(Debug, Clone)]
enum Message {
    Search,
    SearchTextChange(String),
}

struct Explorer {
    search_text: String,
    labels: Option<Vec<(String, IconData)>>,
}

impl Application for Explorer {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_falgs: Self::Flags) -> (Self, Command<Message>) {
        (
            Explorer {
                search_text: "".to_owned(),
                labels: None,
            },
            Command::none(),
        )
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark.clone()
    }

    fn title(&self) -> String {
        "Explorer".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::SearchTextChange(value) => self.search_text = value,
            Message::Search => self.labels = get_icons(&self.search_text),
        };

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut content = row!().padding(10).spacing(20).width(Length::Shrink);

        match self.labels.as_ref() {
            Some(labels) => {
                content = labels.iter().fold(content, |content, (name, item)| {
                    item.styles
                        .iter()
                        .fold(content, |content, style| match style.as_str() {
                            "brands" => content.push(
                                column!(
                                    fa_icon_brands(&name, 40.0).color(color!(255, 255, 255)),
                                    text(format!("{}\n({})", name, "brands"))
                                        .horizontal_alignment(Horizontal::Center)
                                )
                                .width(80)
                                .spacing(5)
                                .align_items(iced::Alignment::Center),
                            ),
                            "solid" => content.push(
                                column!(
                                    fa_icon_solid(&name, 40.0).color(color!(255, 255, 255)),
                                    text(format!("{}\n({})", name, "solid"))
                                        .horizontal_alignment(Horizontal::Center)
                                )
                                .width(80)
                                .spacing(5)
                                .align_items(iced::Alignment::Center),
                            ),
                            _ => content
                                .push(
                                    column!(
                                        fa_icon(&name, 40.0).color(color!(255, 255, 255)),
                                        text(name).horizontal_alignment(Horizontal::Center)
                                    )
                                    .width(80)
                                    .spacing(5),
                                )
                                .align_items(iced::Alignment::Center),
                        })
                });
            }
            None => content = content.push(text("No icon found")),
        };

        column!(
            row!(
                text_input("icon's label", &self.search_text)
                    .on_input(Message::SearchTextChange)
                    .on_submit(Message::Search),
                button(fa_icon_solid("magnifying-glass", 20.0).color(color!(255, 255, 255)))
                    .on_press(Message::Search)
            )
            .align_items(iced::Alignment::Center)
            .spacing(10),
            Scrollable::new(content)
                .direction(Direction::Horizontal(Properties::default()))
                .height(Length::Fill)
        )
        .padding(10)
        .spacing(10)
        .into()
    }
}

#[derive(Debug, Deserialize, Clone)]
struct IconData {
    label: String,
    styles: Vec<String>,
    search: SearchTerms,
}

#[derive(Debug, Deserialize, Clone)]
struct SearchTerms {
    terms: Vec<String>,
}

static INIT: Once = Once::new();
static ICONS_FILE_DATA: &str = include_str!("../assets/font-awesome/icons-light.json");
static mut ICONS_DATA: Option<Mutex<Vec<IconData>>> = None;

fn get_icons_data() -> &'static Mutex<Vec<IconData>> {
    unsafe {
        INIT.call_once(|| {
            let data: Vec<IconData> =
                serde_json::from_str(ICONS_FILE_DATA).expect("Failed to parse JSON");
            ICONS_DATA = Some(Mutex::new(data));
        });

        ICONS_DATA
            .as_ref()
            .expect("ICONS_DATA should be initialized")
    }
}

fn get_icons(search_term: &str) -> Option<Vec<(String, IconData)>> {
    let icons_data = get_icons_data().lock().unwrap();
    let mut items = vec![];

    for item in icons_data.iter() {
        if item.label.starts_with(search_term)
            || item
                .search
                .terms
                .iter()
                .any(|term| term.starts_with(search_term))
        {
            items.push((item.label.to_owned(), item.clone()));
        }
    }

    if items.is_empty() {
        return None;
    }

    Some(items)
}
