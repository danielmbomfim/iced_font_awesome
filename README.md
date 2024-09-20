# Iced Font Awesome

`iced_font_awesome` is a simple and efficient widget for displaying Font Awesome icons in your [Iced](https://github.com/iced-rs/iced) applications. Customize the size and color of your icons effortlessly.

## Features

- **Easy Integration**: Quickly add Font Awesome icons to your Iced projects.
- **Customizable**: Set the size and color of icons to fit your design needs.

## Getting Started

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
iced = "^0.13"
iced_font_awesome = "0.2.0"
```

### Usage

Here's a basic example of how to use iced_font_awesome:

```rust
use iced::{color, widget::row, Task};
use iced_font_awesome::{fa_icon, fa_icon_brands, fa_icon_solid};

fn main() -> iced::Result {
    iced::application("demo", Demo::update, Demo::view).run_with(Demo::new)
}

struct Demo {}

impl Demo {
    fn new() -> (Self, Task<()>) {
        (Self {}, Task::none())
    }

    fn update(&mut self, _message: ()) {}

    fn view(&self) -> iced::Element<'_, ()> {
        row!(
            fa_icon("circle-user").size(40.0).color(color!(0, 255, 0)),
            fa_icon_solid("medal")
                .size(50.0)
                .color(color!(249, 170, 51)),
            fa_icon_brands("google").size(60.0)
        )
        .align_y(iced::Alignment::Center)
        .padding(10)
        .spacing(10)
        .into()
    }
}
```

Result:

![Captura de imagem_20240903_201509](https://github.com/user-attachments/assets/2d4d0e80-5c0d-447a-b180-9025562a4d11)

### Customization

- **Size**: Adjust the size of the icon using the `size` method.
- **Color**: Set the color using the `color` method.

### Icons Explorer

The Icons Explorer is a handy tool to browse and experiment with all available Font Awesome icons.

![Captura de imagem_20240903_201952](https://github.com/user-attachments/assets/c8df48d6-ed91-4eb7-82e3-b5e2673c215c)

To run the Icons Explorer example, follow these steps:

1. Clone the repository:

```sh
git clone https://github.com/danielmbomfim/iced_font_awesome.git
cd iced_font_awesome
```

2. Run the example:

```sh
cargo run --example explorer
```

This will start the Explorer, allowing you to browse through the icons.

### Map of iced version to required iced_font_awesome version.

| Iced Version | Required Iced Font Awesome Version |
| ------------ | ---------------------------------- |
| 0.12         | 0.1.0                              |
| 0.13         | 0.2.0                              |
