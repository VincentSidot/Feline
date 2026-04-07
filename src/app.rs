/* Libs */
use anyhow::Result;
use egui::{Context, Response};

pub type ApplicationId = u64;
pub type ApplicationRenderRet = Option<Response>;

pub trait App {
    /// Called when the application should update its state.
    fn render(&mut self, ctx: &Context) -> ApplicationRenderRet;

    /// Called on application registeration.
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called on application de-registeration.
    fn deinit(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called per frame to determine if the application should be closed.
    fn should_close(&self) -> bool {
        false
    }
}
