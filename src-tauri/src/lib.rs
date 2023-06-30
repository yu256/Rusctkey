use tauri::{App, Manager};
mod services;

pub use crate::services::{
    commands::{check_is_logged_in, fetch_notes, post, set_credentials, upload_files},
    streaming::streaming,
};

#[cfg(mobile)]
mod mobile;

#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }

    pub fn run(self) {
        tauri::Builder::default()
            .setup(move |app| {
                let app_handle = app.app_handle();
                std::thread::spawn(move || {
                    tauri::async_runtime::spawn(streaming(app_handle));
                });
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                check_is_logged_in,
                set_credentials,
                post,
                upload_files,
                fetch_notes
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
