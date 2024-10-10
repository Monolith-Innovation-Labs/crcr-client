// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;
use rand::prelude::SliceRandom;
use slint::{Model, PhysicalSize};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.window().set_size(PhysicalSize::new(400, 660));

    ui.run()?;
    Ok(())
}
