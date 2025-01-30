use std::sync::{Mutex, Once};

use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{button, column, row, text, text_input, Scrollable};
use iced::{color, Alignment, Element, Length, Size, Task, Theme};
use iced_font_awesome::{fa_icon, fa_icon_brands, fa_icon_solid};
use serde::Deserialize;

pub fn main() -> iced::Result {
    iced::application("Explorer", Explorer::update, Explorer::view)
        .window_size(Size::new(800.0, 300.0))
        .theme(Explorer::theme)
        .run_with(Explorer::new)
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

impl Explorer {
    fn new() -> (Self, Task<Message>) {
        (
            Explorer {
                search_text: "".to_owned(),
                labels: None,
            },
            Task::none(),
        )
    }

    fn theme(&self) -> Theme {
        Theme::Dark.clone()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchTextChange(value) => self.search_text = value,
            Message::Search => self.labels = get_icons(&self.search_text),
        };

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let mut content = row!().padding(10).spacing(20).width(Length::Shrink);

        match self.labels.as_ref() {
            Some(labels) => {
                content = labels.iter().fold(content, |content, (name, item)| {
                    item.styles
                        .iter()
                        .fold(content, |content, style| match style.as_str() {
                            "brands" => content.push(
                                column!(
                                    fa_icon_brands(&name)
                                        .size(40.0)
                                        .color(color!(255, 255, 255)),
                                    text!("{}\n({})", name, "brands").align_x(Alignment::Center)
                                )
                                .width(80)
                                .spacing(5)
                                .align_x(Alignment::Center),
                            ),
                            "solid" => content.push(
                                column!(
                                    fa_icon_solid(&name).size(40.0).color(color!(255, 255, 255)),
                                    text!("{}\n({})", name, "solid").align_x(Alignment::Center)
                                )
                                .width(80)
                                .spacing(5)
                                .align_x(Alignment::Center),
                            ),
                            _ => content.push(
                                column!(
                                    fa_icon(&name).size(40.0).color(color!(255, 255, 255)),
                                    text!("{}", name).align_x(Alignment::Center)
                                )
                                .align_x(Alignment::Center)
                                .width(80)
                                .spacing(5),
                            ),
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
                button(
                    fa_icon_solid("magnifying-glass")
                        .size(20.0)
                        .color(color!(255, 255, 255))
                )
                .on_press(Message::Search)
            )
            .align_y(Alignment::Center)
            .spacing(10),
            Scrollable::new(content)
                .direction(Direction::Horizontal(Scrollbar::default()))
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

#[allow(static_mut_refs)]
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
