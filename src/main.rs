use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill, Theme};

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Terminux")
        .theme(Theme::Dark)
        .run()
}

#[derive(Default)]
struct App {
    split: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleSplit,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleSplit => {
                self.split = !self.split;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let primary = container(text("Terminal A"))
            .style(pane_style)
            .width(Fill)
            .height(Fill)
            .padding(20);
        

        let body: Element<'_, Message> = if self.split {
            row![
                primary,
                container(text("Terminal B"))
                    .style(pane_style)
                    .width(Fill)
                    .height(Fill)
                    .padding(20)
            ]
            .spacing(12)
            .into()
        } else {
            primary.into()
        };

        column![
            row![
                text("Terminux").size(24),
                button("Toggle split").on_press(Message::ToggleSplit)
            ]
            .spacing(16),
            body
        ]
        .padding(16)
        .spacing(12)
        .into()
    }
}

fn pane_style(_: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(
            iced::Color::from_rgb8(34, 39, 49),
        )), 
        border: iced::Border {
            color: iced::Color::from_rgb8(90, 105, 130),
            width: 1.0, 
            radius: 12.0.into(),
        }, 
        ..Default::default()
    }
}
