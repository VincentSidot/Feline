use log::Level;
use std::{env, sync::OnceLock};

use crate::utils::logger::logfmt;

pub fn load_env() {
    static LOAD_ONCE: OnceLock<()> = OnceLock::new();

    LOAD_ONCE.get_or_init(|| {
        if let Err(err) = {
            if let Some(env_file) = env::var_os("ENV_FILE") {
                let text = logfmt(
                    Level::Info,
                    "env",
                    file!(),
                    line!(),
                    &format_args!("Loading custom env file from '{}'", env_file.display()),
                );
                println!("{}", text);

                dotenvy::from_filename(env_file)
            } else {
                dotenvy::dotenv()
            }
        } {
            let text = logfmt(
                Level::Warn,
                "env",
                file!(),
                line!(),
                &format_args!("An error occurred while loading the .env file ({})", err),
            );
            println!("{}", text);
        }
    });
}
