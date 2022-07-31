#![warn(clippy::all, rust_2018_idioms)]

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    wave_framework::WaveApp::run();
}
