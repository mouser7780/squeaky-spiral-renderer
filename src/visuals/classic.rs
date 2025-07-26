use nannou::prelude::*;
use nannou_egui::egui;
use nannou_egui::egui::Ui;
use crate::visuals::{Visual, u83_to_rgb};

pub struct ClassicVisual {
    points: Vec<Point2>,  // stores cartesian coords of all points on the spiral polygon
    offset: f32, // offsets the start position of the spiral, only really useful for debugging purposes
    count: i32, // the number of points on the spiral in total
    turns: i32, // the number of turns in the spiral, how "twisted" it is
    width: f32, // the weighting of each color in the spiral
    angle_max: f32, // the upper limit of the angles the spiral calculates for
    warp: f32, // determines how big the center of the spiral is in proportion to the rest of the spiral
    rot: f32, // the amount the spiral rotates by each frame, dependent on speed
    rad_max: f32, // the maximum radius of the spiral, dependent on angle_max
    cur_coefficient: f32, // the curvature coefficient of the spiral, dependent on warp
    x: Vec<f32>, // temporarily stores the x coordinates for the points before being combined
    y: Vec<f32>, // temporarily stores the y coordinates for the points before being combined
    // imported vars
    speed: f32,
    needs_update: bool,
    w: u32,
    h: u32,
    color1: [u8; 3],
    color2: [u8; 3],
}

impl ClassicVisual {
    pub fn new(w: u32, h: u32) -> Self {
        let offset = ((((h as f32) / 2.0) / ((w as f32) / 2.0)).atan()) / (2.0 * PI);
        let turns = 5;
        let count = 720;
        let width = 0.5;
        let warp = 0.0;
        let cur_coefficient = 10.0_f32.pow(warp);
        let angle_max = ((2.0*PI*turns as f32*((w as f32/2.0).pow(2.0)+(h as f32/2.0).pow(2.0)).sqrt())/(1000.0*(1.0-(width/turns as f32)).pow(cur_coefficient))).pow(1.0/cur_coefficient);
        let rad_max = (1.0/(2.0*PI*turns as f32))*angle_max.pow(cur_coefficient);
        
        Self {
            offset,
            rot: 0.0,
            warp,
            turns,
            count,
            width,
            angle_max,
            rad_max,
            cur_coefficient,
            x: Vec::new(),
            y: Vec::new(),
            points: Vec::new(),
            speed: 0.2,
            needs_update: true,
            w,
            h,
            color1: [0,0,0],
            color2: [255,255,255],
        }
    }
    
    fn recalculate_geometry(&mut self) {
        self.cur_coefficient = 10.0f32.pow(self.warp);
        self.angle_max = ((2.0 * PI * self.turns as f32 * ((self.w as f32/ 2.0).pow(2.0) + (self.h as f32 /2.0).pow(2.0)).sqrt()) / (1000.0 * (1.0 - (self.width / self.turns as f32)).pow(self.cur_coefficient))).pow(1.0 / self.cur_coefficient);
        self.rad_max = (1.0 /(2.0 * PI * self.turns as f32)) * self.angle_max.pow(self.cur_coefficient);
    }
}

impl Visual for ClassicVisual {
    fn name(&self) -> &str {
        "Classic Spiral"
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
        self.rot += self.speed * delta_time;
        if self.needs_update {
            self.recalculate_geometry();
            self.needs_update = false;
        }
        self.x.clear();
        self.y.clear();
        
        
        // outer spiral edge points
        for i in 0..=self.count {
            let cur = i as f32 / self.count as f32;
            let cur_rot = 2.0 *PI * (cur * self.turns as f32 + self.offset + self.rot);
            let cur_rad = self.rad_max * cur.pow(self.cur_coefficient);
            self.x.push(cur_rad * cur_rot.cos());
            self.y.push(cur_rad * cur_rot.sin());
        }
        // inner spiral edge (reversed) points
        for i in (0..=(self.count as f32 * (1.0 - (self.width / self.turns as f32))) as i32).rev() {
            let cur = i as f32 / self.count as f32;
            let cur_rot = 2.0 * PI * (cur * (self.turns as f32) + self.offset + self.rot - self.width);
            let cur_rad = self.rad_max * cur.pow(self.cur_coefficient);
            self.x.push(cur_rad * cur_rot.cos());
            self.y.push(cur_rad * cur_rot.sin());
        }
        // collecting points into one vector
        self.points = self.x
            .iter()
            .zip(self.y.iter())
            .map( | ( & x, & y) | pt2(x * 1000.0, y * 1000.0))
            .collect();
    }
    fn draw(&self, draw: &Draw) {
        let colour1rgb = u83_to_rgb(self.color1);
        let colour2rgb = u83_to_rgb(self.color2);
        draw.background().color(colour1rgb);
        draw.polygon()
            .color(colour2rgb)
            .points(self.points.iter().cloned());
    }
    fn ui(&mut self, ui: &mut Ui) {
        self.needs_update |= ui.add(egui::Slider::new( & mut self.turns, 1..=15).text("Turns")).changed();
        ui.add(egui::Slider::new( & mut self.speed, - 10.0..=10.0).text("Speed"));
        self.needs_update |= ui.add(egui::Slider::new( & mut self.warp, - 0.5..=0.5).text("Warp")).changed();
        ui.add(egui::Slider::new( & mut self.count, 90..=1080).text("Resolution"));
        ui.label("Color 1");
        ui.color_edit_button_srgb( & mut self.color1);
        ui.label("Color 2");
        ui.color_edit_button_srgb( & mut self.color2);
    }
}