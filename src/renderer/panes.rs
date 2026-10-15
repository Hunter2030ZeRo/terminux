use crate::app::{App, Message};
use crate::renderer::theme::pane_style;
use iced::widget::{button, column, container, pane_grid, row, text};
use iced::{Element, Fill};

pub fn view(app: &App) -> Element<'_, Message> {
    let active = app.active;
    let grid = pane_grid::PaneGrid::new(&app.panes, |_pane, content, _maximized| {
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
