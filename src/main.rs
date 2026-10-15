use terminux::app::App;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Terminux")
        .theme(iced::Theme::Dark)
        .subscription(App::subscription)
        .run()
}











