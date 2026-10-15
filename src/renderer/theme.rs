pub fn pane_style(_: &iced::Theme, active: bool) -> iced::widget::container::Style {
    let border_color = if active {
        iced::Color::from_rgb8(130, 170, 255)
    } else {
        iced::Color::from_rgb8(90, 105, 130)
    };

    iced::widget::container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb8(34, 39, 49))),
        border: iced::Border {
            color: border_color,
            width: if active { 2.0 } else { 1.0 },
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}
