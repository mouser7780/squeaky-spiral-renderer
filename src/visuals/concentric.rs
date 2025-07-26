use nannou::prelude::*;
use nannou_egui::egui;
use nannou_egui::egui::Ui;
use crate::visuals::{Visual, u83_to_rgb};

pub struct ConcentricVisual {
    count: i32,
    rad_max: f32,
    rad_min: f32,
    width: f32,
    turns: i32,
    speed: f32,
    scale: f32,
    w: u32,
    h: u32,
    color1: [u8; 3],
    color2: [u8; 3],
    circles: Vec<Vec<Point2>>,
    needs_update: bool,
}

impl ConcentricVisual {
    pub fn new(w: u32, h: u32) -> Self {
        let rad_max=(((w as f32/ 2.0).pow(2.0) + (h as f32 /2.0).pow(2.0)).sqrt())/1000.0;
        Self{
            circles: Vec::new(),
            count:720,
            rad_max,
            rad_min:0.0001,
            width:0.5,
            turns:3,
            speed:0.2,
            scale:0.0,
            w,
            h,
            color1: [0,0,0],
            color2: [255,255,255],
            needs_update:true,
        }
    }
    
    fn recalculate_geometry(&mut self) {
        self.rad_max=(((self.w as f32/ 2.0).pow(2.0) + (self.h as f32 /2.0).pow(2.0)).sqrt())/1000.0;
    }
}

impl Visual for ConcentricVisual {
    fn name(&self) -> &str {
        "Concentric Shapes"
    }
    fn setup(&mut self, _w: u32, _h: u32, _speed: f32, _needs_update: bool, _color1: [u8; 3], _color2: [u8; 3]) {
        self.recalculate_geometry();
        self.needs_update = false;
    }

    fn resize(&mut self, w: u32, h: u32) {
        self.w = w;
        self.h = h;

        self.recalculate_geometry();
        self.needs_update = true;
    }
    
    fn update(&mut self, delta_time: f32) {
        self.scale += self.speed * delta_time;

        self.circles.clear();

        if self.needs_update {
            self.recalculate_geometry();
            self.needs_update = false;
        }

        for i in 1..self.turns+3 {
            let range = self.rad_max - self.rad_min;
            let spacing = range / (self.turns as f32 + 1.5);

            let base = self.rad_min + spacing * i as f32;
            let half_width = (self.width * spacing) / 2.0;
            let radius_center = wrap_range(base + self.scale, self.rad_min-half_width, self.rad_max+half_width);
            
            let inner_rad = (radius_center - half_width).max(self.rad_min);
            let outer_rad = (radius_center + half_width).min(self.rad_max);

            let mut inner_ring = Vec::new();
            let mut outer_ring = Vec::new();

            for j in 0..self.count {
                let angle = 2.0 * PI * j as f32 / self.count as f32;
                inner_ring.push(pt2(inner_rad * angle.cos() * 1000.0, inner_rad * angle.sin() * 1000.0));
                outer_ring.push(pt2(outer_rad * angle.cos() * 1000.0, outer_rad * angle.sin() * 1000.0));
            }
            inner_ring.push(pt2(inner_rad * 1000.0, 0.0));
            outer_ring.push(pt2(outer_rad * 1000.0, 0.0));

            let mut ring_points = outer_ring.clone();
            ring_points.extend(inner_ring.into_iter());
            self.circles.push(ring_points);
        }
    }


    fn draw(&self, draw: &Draw) {
        let colour1rgb = u83_to_rgb(self.color1);
        let colour2rgb = u83_to_rgb(self.color2);
        draw.background().color(colour1rgb);
        for ring in &self.circles {
            draw.polygon()
                .color(colour2rgb)
                .points(ring.clone());
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        self.needs_update |= ui.add(egui::Slider::new(&mut self.count, 3..=1080).text("Resolution")).changed();
        self.needs_update |= ui.add(egui::Slider::new(&mut self.turns, 1..=50).text("Turns")).changed();
        self.needs_update |= ui.add(egui::Slider::new(&mut self.width, 0.01..=1.00).text("Width")).changed();
        self.needs_update |= ui.add(egui::Slider::new(&mut self.speed , -1.00..=1.00).text("Speed")).changed();
        ui.label("Color 1");
        ui.color_edit_button_srgb( & mut self.color1);
        ui.label("Color 2");
        ui.color_edit_button_srgb( & mut self.color2);
    }
}

fn wrap_range(value: f32, min: f32, max: f32) -> f32 {
    let range = max - min;
    let mut v = value - min;
    v = v - (v / range).floor() * range;
    v + min
}