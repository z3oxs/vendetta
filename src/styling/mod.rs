use gtk::prelude::*;
use gtk::CssProvider;
use crate::utils::types::Config;

pub struct Provider {
    pub config: Config,
}

impl Provider {
    pub fn new(&self) -> CssProvider {
        let mut css: String = Self::create_default_css();

        if self.config.window.opacity < 1.0 {
            css = Self::create_transparent_css();

        }

        let provider = gtk::CssProvider::new();

        provider.load_from_data(css.as_bytes())
            .expect("Failed to read custom CSS");

        provider
    }

    fn create_transparent_css() -> String {
        format!("
            .button {{
                border: none;
                background-color: transparent;
            }}

            .button:hover {{
                transition: 300ms linear;
                background-color: rgba(0, 0, 0, 0.25);
            }}
        ")
    }

    fn create_default_css() -> String {
        format!("
            .button {{
                border: none;
            }}

            .button:hover {{
                transition: 300ms linear;
            }}
        ")
    }
}
