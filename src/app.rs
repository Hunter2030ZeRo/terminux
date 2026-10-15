use iced::widget::{button, column, container, pane_grid, row, text};
use iced::{Element, Fill, Theme};

#[derive(Debug, Clone)]
enum Message {
    Split,
    Activate(pane_grid::Pane),
    Resize(pane_grid::ResizeEvent),
    CloseActive,
    ToggleAxis,
    Terminal(iced_term::Event),
}


struct App {
    panes: pane_grid::State<PaneContent>,
    active: pane_grid::Pane,
    next_id: usize,
    split_axis: pane_grid::Axis,
}

impl Default for App {
    fn default() -> Self {
        let (panes, first_pane) =
            pane_grid::State::new(PaneContent::new(1).expect("Failed to create terminal"));

        Self {
            panes,
            active: first_pane,
            next_id: 2,
            split_axis: pane_grid::Axis::Vertical,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Split => {
                let id = self.next_id as u64;

                match PaneContent::new(id) {
                    Ok(content) => {
                        if let Some((new_pane, _)) =
                            self.panes.split(self.split_axis, self.active, content)
                        {
                            self.active = new_pane;
                            self.next_id += 1;
                        }
                    }
                    Err(error) => {
                        eprintln!("Failed to create terminal: {error}");
                    }
                }
            }

            Message::Activate(pane) => {
                self.active = pane;
            }

            Message::Resize(event) => {
                self.panes.resize(event.split, event.ratio);
            }

            Message::CloseActive => {
                if let Some((_, sibling)) = self.panes.close(self.active) {
                    self.active = sibling;
                }
            }

            Message::ToggleAxis => {
                self.split_axis = match self.split_axis {
                    pane_grid::Axis::Horizontal => pane_grid::Axis::Vertical,
                    pane_grid::Axis::Vertical => pane_grid::Axis::Horizontal,
                };
            }

            Message::Terminal(iced_term::Event::BackendCall(id, command)) => {
                if let Some((_, content)) = self
                    .panes
                    .iter_mut()
                    .find(|(_, content)| content.terminal.id == id)
                {
                    let action = content
                        .terminal
                        .handle(iced_term::Command::ProxyToBackend(command));

                    match action {
                        iced_term::actions::Action::ChangeTitle(title) => {
                            content.title = title;
                        }

                        iced_term::actions::Action::Shutdown => {}

                        _ => {}
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let active = self.active;
        let grid = pane_grid::PaneGrid::new(&self.panes, |_pane, content, _maximized| {
            pane_grid::Content::new(
                container(iced_term::TerminalView::show(&content.terminal).map(Message::Terminal))
                    .width(Fill)
                    .height(Fill)
                    .padding(6)
                    .style(move |theme| pane_style(theme, _pane == active)),
            )
        })
        .spacing(12)
        .on_click(Message::Activate)
        .on_resize(10, Message::Resize);

        column![
            row![
                text("Terminux").size(24),
                button("Toggle split").on_press(Message::Split),
                button("Close").on_press(Message::CloseActive),
                button("Toggle axis").on_press(Message::ToggleAxis),
            ]
            .spacing(16),
            grid
        ]
        .padding(16)
        .spacing(12)
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(
            self.panes
                .iter()
                .map(|(_, content)| content.terminal.subscription().map(Message::Terminal)),
        )
    }
}
