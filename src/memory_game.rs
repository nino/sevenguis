use std::rc::Rc;

slint::include_modules!();

pub fn make_memory_game() -> Result<MemoryGame, slint::PlatformError> {
    use slint::Model;

    let ui = MemoryGame::new()?;
    let mut tiles: Vec<TileData> = ui.get_memory_tiles().iter().collect();
    tiles.extend(tiles.clone());

    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    let tiles_model = Rc::new(slint::VecModel::from(tiles));
    ui.set_memory_tiles(tiles_model.clone().into());

    let handle = ui.as_weak();
    ui.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved);
        if let (Some((t1_index, mut t1)), Some((t2_index, mut t2))) =
            (flipped_tiles.next(), flipped_tiles.next())
        {
            let pair_is_solved = t1 == t2;
            if pair_is_solved {
                t1.solved = true;
                t2.solved = true;
                tiles_model.set_row_data(t1_index, t1);
                tiles_model.set_row_data(t2_index, t2);
            } else {
                let main_window = handle.unwrap();
                main_window.set_disable_tiles(true);
                let tiles_model = tiles_model.clone();
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    t2.image_visible = false;
                    tiles_model.set_row_data(t1_index, t1);
                    tiles_model.set_row_data(t2_index, t2);
                })
            }
        }
    });

    Ok(ui)
}
