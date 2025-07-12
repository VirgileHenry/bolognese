mod common;
mod constraint;
mod factorio;
mod grid;
mod library;
// mod quantic;
// mod wfc_cell;

fn main() {
    let game_libray = match library::Library::new("./data/") {
        Ok(lib) => lib,
        Err(e) => {
            println!("Failed to load game library: {e}");
            std::process::exit(1);
        }
    };

    let lib_info = game_libray.info();
    println!("{lib_info}");
}
