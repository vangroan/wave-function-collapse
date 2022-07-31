#![warn(clippy::all, rust_2018_idioms)]

use wave_framework::load_tileset_file;

const TILESET_DIR: &str = "vendor/WaveFunctionCollapse/tilesets";

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    load_tileset_file(format!("{}/Castle.xml", TILESET_DIR).as_str()).unwrap();

    wave_framework::WaveApp::run();
}
