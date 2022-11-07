mod app;
mod utils;
mod config;
mod styling;

use gtk::Application;

fn main() {
    let config = config::parse();

    let app = app::App {
        app: Application::builder()
            .application_id("com.z3oxs.vendetta")
            .build(),

        config,
    };

    app.run();
}
