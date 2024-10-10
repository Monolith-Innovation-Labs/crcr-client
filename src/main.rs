// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use rand::prelude::SliceRandom;
use slint::{Model, PhysicalSize, SharedString};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.global::<TextboxLogic>().on_set_currsor(|cursor, text| {
        let cursor = cursor.clamp(0, text.len() as i32) as usize;
        let before = &text[0..cursor];
        let after = &text[cursor..text.len()];

        return SharedString::from(before.to_owned() + "|" + after);
    });

    ui.global::<TextboxLogic>().on_str_len(|text| {
        return text.len() as i32;
    });

    ui.global::<TextboxLogic>().on_remove_after_currsor(|cursor, text| {
        let cursor = cursor.clamp(0, text.len() as i32) as usize;

        if cursor == text.len() {
            return text;
        }

        let before = &text[0..cursor];
        let after = &text[cursor + 1 ..text.len()];

        return SharedString::from(before.to_owned() + after);
    });

    ui.global::<TextboxLogic>().on_remove_before_currsor(|cursor, text| {
        let cursor = cursor.clamp(0, text.len() as i32) as usize;

        if cursor == 0 {
            return text;
        }

        let before = &text[0..cursor-1];
        let after = &text[cursor..text.len()];

        return SharedString::from(before.to_owned() + after);
    });

    ui.global::<TextboxLogic>().on_add_char_currsor(|cursor, text, char| {
        let cursor = cursor.clamp(0, text.len() as i32) as usize;

        if char.len() == 0 || !char.is_ascii() || !char.chars().all(|char| char.is_alphanumeric() || char == ' ' || char.is_ascii_punctuation())  {
            return text;
        }

        let before = &text[0..cursor];
        let after = &text[cursor ..text.len()];

        return SharedString::from(before.to_owned() + &char + after);
    });

    ui.window().set_size(PhysicalSize::new(400, 660));
    ui.run()?;
    Ok(())
}
