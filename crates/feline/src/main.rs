mod utils;

/* Libs */
use anyhow::Result;

/* Locals */
use feline_render::WinitApplication;
use feline_ui::FelineUi;

fn main() -> Result<()> {
    utils::logger::init();
    let mut app = WinitApplication::default();
    app.register_default::<FelineUi>();
    app.run()
}
