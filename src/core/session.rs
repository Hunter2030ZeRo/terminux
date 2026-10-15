pub struct PaneContent {
    pub title: String,
    pub terminal: iced_term::Terminal,
}

impl PaneContent {
    pub fn new(id: u64) -> std::io::Result<Self> {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".into());

        let settings = iced_term::settings::Settings {
            backend: iced_term::settings::BackendSettings {
                program: shell,
                ..Default::default()
            },
            ..Default::default()
        };

        let terminal = iced_term::Terminal::new(id, settings)?;

        Ok(Self {
            title: format!("Terminal {id}"),
            terminal,
        })
    }
}
