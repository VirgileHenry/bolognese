mod common;
mod constraint;
mod factorio;
mod grid;
mod library;
mod quantic;

fn main() {
    let game_libray = library::Library::default();
    match game_libray.ensure_integrity() {
        Ok(_) => {}
        Err(e) => {
            println!("Game library integrity failed: {e}");
            std::process::exit(1);
        }
    }

    let lib_info = game_libray.info();
    println!("{lib_info}");

    let grid_size = common::Size::new(20, 10).unwrap();
    let mut wfc_grid = grid::Grid::from_fn(20, 10, |grid_pos| {
        grid::TileState::Supperposed(quantic::Tile::all(&game_libray, grid_pos, grid_size))
    });

    loop {
        match wfc_grid.collapse_once() {
            Ok(true) => continue,
            Ok(false) => break,
            Err(e) => {
                println!("Failed to collapse: {e}");
                break;
            }
        }
    }

    wfc_grid.display();
}
