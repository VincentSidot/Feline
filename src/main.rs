/* Models */
mod app;
mod constants;
mod platform;
mod state;
mod ui;
mod utils;

/* Libs */
use anyhow::Result;

fn main() -> Result<()> {
    utils::logger::init();
    app::run()
}
