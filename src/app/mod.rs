mod transparency;

use std::process::{Command, exit};
use std::env::var;

use gtk::prelude::*;
use gtk::{
    Application,
    ApplicationWindow,
    Box,
    Orientation,
    Align,
    Image,
    Button,
};

use crate::utils::types::Config;
use crate::styling::Provider;
use crate::app::transparency::*;

pub struct App {
    pub app: Application,
    pub config: Config,
}

impl App {
    pub fn run(self) {
        self.app.connect_activate(move | app | {
            let provider = Provider {
                config: self.config.clone(),
            }
                .new();

            let window = ApplicationWindow::builder()
                .application(app)
                .default_width(280)
                .default_height(100)
                .resizable(false)
                .has_focus(true)
                .window_position(gtk::WindowPosition::CenterAlways)
                .decorated(false)
                .title("Vendetta")
                .build();

            if self.config.window.opacity < 1.0 {
                window.set_app_paintable(true);

                set_visual(&window, None);

                window.connect_screen_changed(set_visual);

                let config_clone = self.config.clone();

                window.connect_draw(move | window, ctx | {
                    draw(window, ctx, &config_clone)
                });
            }

            window
                .style_context()
                .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

            let buttons = Box::builder()
                .orientation(Orientation::Horizontal)
                .spacing(20)
                .valign(Align::Center)
                .halign(Align::Center)
                .build();

            let shutdown = Self::create_button("power", &provider);
            let reboot = Self::create_button("restart", &provider);
            let logout = Self::create_button("logout", &provider);

            buttons.add(&shutdown);
            buttons.add(&reboot);
            buttons.add(&logout);

            let config_clone = self.config.clone();

            shutdown.connect_clicked(move | _ | Self::click(&config_clone.commands.shutdown));
            logout.connect_clicked(move | _ | Self::click(&config_clone.commands.logout));
            reboot.connect_clicked(move | _ | Self::click(&config_clone.commands.reboot));

            window.add(&buttons);

            window.connect_focus_out_event(| window, _ | {
                if !window.has_focus() {
                    exit(2);

                }

                Inhibit(false)
            });

            window.connect_key_press_event(move | _, event | {
                if event.keycode() == Some(9) {
                    exit(0);
                
                }

                Inhibit(false)
            });

            window.show_all();
        });
            
        self.app.run_with_args(&[""]);
    }

    fn create_button(image: &'static str, provider: &gtk::CssProvider) -> gtk::Button {
        let home = match var("HOME") {
            Ok(var) => var,
            Err(_) => {
                println!("\x1b[31m[X] Failed to get HOME environment variable.\x1b[m");

                exit(2);
            },
        };

        let button = Button::builder()
            .image(&Image::builder()
                .file(&format!("{home}/.config/vendetta/{image}.svg"))
                .build()
            )
            .always_show_image(true)
            .image_position(gtk::PositionType::Top)
            .width_request(70)
            .height_request(70)
            .build();

        button
            .style_context()
            .add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        button
            .style_context()
            .add_class("button");

        button
    }

    fn click(command: &String) {
        let args = command
            .split(" ")
            .collect::<Vec<&str>>();

        let mut cmd = Command::new(args[0]);

        if command.len() > 1 {
            cmd
                .args(&args[1..]);
        }

        cmd
            .spawn()
            .expect("Failed to run command");
    }
}
