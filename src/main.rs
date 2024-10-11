// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod textbox;
mod chat;

use crate::textbox::connect_textbox;
use slint::PhysicalSize;
use std::error::Error;
use crate::chat::connect_chat_component;

slint::include_modules!();

#[tokio::main]
async fn main() {
    let ui = AppWindow::new().unwrap();

    connect_textbox(&ui);
    connect_chat_component(&ui);

    ui.window().set_size(PhysicalSize::new(400, 720));
    ui.run().unwrap();
}
