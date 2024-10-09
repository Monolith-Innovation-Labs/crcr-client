// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;
use rand::prelude::SliceRandom;
use slint::Model;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let mut tiles = ui.get_memory_tiles().iter().collect::<Vec<TileData>>();
    tiles.extend(tiles.clone());

    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    let tiles_model = Rc::new(slint::VecModel::from(tiles));
    ui.set_memory_tiles(tiles_model.clone().into());

    let ui_weak = ui.as_weak();
    ui.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) = (flipped_tiles.next(), flipped_tiles.next()) {
            let is_pair_solved = t1 == t2;

            if is_pair_solved {
                t1.solved = true;
                t2.solved = true;
                tiles_model.set_row_data(t1_idx, t1);
                tiles_model.set_row_data(t2_idx, t2);
            }
            else {
                let ui = ui_weak.unwrap();

                ui.set_disable_tiles(true);
                let tiles_model = tiles_model.clone();
                slint::Timer::single_shot(std::time::Duration::from_millis(300), move || {
                    ui.set_disable_tiles(false);
                    t1.image_visible = false;
                    t2.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    tiles_model.set_row_data(t2_idx, t2);
                });
            }
        }
    });

    ui.run()?;
    Ok(())
}
