use iced::widget::{button, column, container, pane_grid, row, text};
use iced::{Element, Fill, Theme};

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Terminux")
        .theme(Theme::Dark)
        .run()
}

struct App {
    panes: pane_grid::State<PaneContent>,
    active: pane_grid::Pane,
    next_id: usize,
}

struct PaneContent {
    title: String,
}

#[derive(Debug, Clone)]
enum Message {
    Split,
    Activate(pane_grid::Pane),
    Resize(pane_grid::ResizeEvent),
}

impl Default for App {
    fn default() -> Self {
        let (panes, first_pane) = pane_grid::State::new(PaneContent {
            title: "Terminal 1".into(),
        });

        Self {
            panes,
            active: first_pane,
            next_id: 2,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Split => {
                let title = format!("Terminal {}", self.next_id);

                if let Some((new_pane, _)) = self.panes.split(
                    pane_grid::Axis::Horizontal,
                    self.active,
                    PaneContent { title },
                ) {
                    self.active = new_pane;
                    self.next_id += 1;
                }
            }

            Message::Activate(pane) => {
                self.active = pane;
            }

            Message::Resize(event) => {
                self.panes.resize(event.split, event.ratio);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let grid = pane_grid::PaneGrid::new(&self.panes, |_pane, content, _maximized| {
            pane_grid::Content::new(
                container(text(content.title.as_str()))
                    .width(Fill)
                    .height(Fill)
                    .padding(20)
                    .style(pane_style),
            )
        })
        .spacing(12)
        .on_click(Message::Activate)
        .on_resize(10, Message::Resize);

        column![
            row![
                text("Terminux").size(24),
                button("Toggle split").on_press(Message::Split)
            ]
            .spacing(16),
            grid
        ]
        .padding(16)
        .spacing(12)
        .into()
    }
}

fn pane_style(_: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb8(34, 39, 49))),
        border: iced::Border {
            color: iced::Color::from_rgb8(90, 105, 130),
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}
