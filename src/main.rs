mod app;
mod core;
mod renderer;

use crate::app::App;

fn main() -> iced::Result {
    iced::application(App::default, App::update, renderer::panes::view)
        .title("Terminux")
        .theme(iced::Theme::Dark)
        .subscription(App::subscription)
        .run()
}
