use nannou::prelude::*;
use nannou_egui::egui;
pub trait Visual {
    fn name(&self) -> &str;
    fn setup(&mut self, w: u32, h: u32, speed: f32, needs_update: bool, color1: [u8; 3], color2: [u8; 3]);
    fn resize(&mut self, w: u32, h: u32);
    fn update(&mut self, delta_time: f32);
    fn draw (&self, draw: &Draw);
    fn ui (&mut self, ui: &mut egui::Ui);
}

pub fn u83_to_rgb(color: [u8;3]) -> Rgb {
    rgb(
        color[0] as f32 / 255.0,
        color[1] as f32 / 255.0,
        color[2] as f32 / 255.0,
    )
}

pub mod classic;
pub use classic::ClassicVisual;