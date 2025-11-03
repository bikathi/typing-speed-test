pub(crate) const MAX_FONT_SIZE: i32 = 50i32;
pub(crate) const MIN_FONT_SIZE: i32 = 10i32;
pub(crate) const DEFAULT_TEXT: &str = "Hi dear user!
Welcome to this simple and free typing speed test tool.
Click on this paragraph to start typing. When you type something wrong, it lights up red else it lights up white.
You can upload your own text by clicking the input box above the timer pause/play button. Safe typing!";

#[derive(Debug, Clone)]
pub(crate) enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub(crate) struct AppSettings {
    pub font_size: i32,
    pub theme: Theme,
    pub timer_running: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            font_size: 30,
            theme: Theme::Dark,
            timer_running: false,
        }
    }
}

impl AppSettings {
    pub(crate) fn inc_font_size(&mut self) {
        self.font_size += 10;
    }

    pub(crate) fn dec_font_size(&mut self) {
        self.font_size -= 10;
    }

    pub(crate) fn set_theme(&mut self, new_value: Theme) {
        self.theme = new_value;
    }

    pub(crate) async fn toggle_timer_running(&mut self) {
        self.timer_running = !self.timer_running;
    }
}
