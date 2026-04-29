//! Editor application entry point (desktop-only stub).

/// Placeholder for the desktop editor application.
///
/// This will be replaced by a full egui/Bevy-powered editor in M3.
pub struct EditorApp {
    title: String,
}

impl EditorApp {
    /// Create a new editor application with the given window title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }

    /// Return the window title.
    pub fn title(&self) -> &str {
        &self.title
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_app_stores_title() {
        let app = EditorApp::new("iGame Editor");
        assert_eq!(app.title(), "iGame Editor");
    }
}
