use crate::core::session::PaneContent;
use crate::renderer::theme::pane_style;
use iced::widget::{button, column, container, pane_grid, row, text};
use iced::{Element, Fill};

#[derive(Debug, Clone)]
pub enum Message {
    Split,
    Activate(pane_grid::Pane),
    Resize(pane_grid::ResizeEvent),
    CloseActive,
    ToggleAxis,
    Terminal(iced_term::Event),
}

pub struct App {
    pub panes: pane_grid::State<PaneContent>,
    pub active: pane_grid::Pane,
    pub next_id: usize,
    pub split_axis: pane_grid::Axis,
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
    pub fn update(&mut self, message: Message) {
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

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(
            self.panes
                .iter()
                .map(|(_, content)| content.terminal.subscription().map(Message::Terminal)),
        )
    }
}
