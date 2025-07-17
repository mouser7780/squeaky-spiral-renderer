use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use std::time::{Instant, Duration};


fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    // window components
    _window: window::Id,
    egui: Egui,
    w:u32,
    h:u32,
    last_frame_time: Instant,
    state: AppState,

    // FPS counter
    fps_timer: Instant,
    frame_count: u32,
    fps: f32,

    // spiral colors
    color1: [u8; 3],
    color2: [u8; 3],

    // spiral variables
    points: Vec<Point2>,  // stores cartesian coords of all points on the spiral polygon
    speed: f32, // stores the rotation speed of the spiral
    offset: f32, // offsets the start position of the spiral, only really useful for debugging purposes
    count: i32, // the number of points on the spiral in total
    turns: i32, // the number of turns in the spiral, how "twisted" it is
    width: f32, // the weighting of each color in the spiral
    angle_max: f32, // the upper limit of the angles the spiral calculates for
    warp: f32, // determines how big the center of the spiral is in proportion to the rest of the spiral

    // misc
    rot: f32, // the amount the spiral rotates by each frame, dependent on speed
    rad_max: f32, // the maximum radius of the spiral, dependent on angle_max
    cur_coefficient: f32, // the curvature coefficient of the spiral, dependent on warp
    needs_update: bool, // tracks whether the spiral generation values need updating this frame
    x: Vec<f32>, // temporarily stores the x coordinates for the points before being combined
    y: Vec<f32>, // temporarily stores the y coordinates for the points before being combined
}
enum AppState {
    MainMenu,
    SpiralView,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .msaa_samples(4)
        .build()
        .unwrap();
    let egui = Egui:: from_window(&app.main_window());

    let color1 = [0,0,0];
    let color2 = [255,255,255];

    let (count, turns, width, cur_coefficient) = (720, 5, 0.5, 1.0);
    let warp =cur_coefficient.log10();
    let (w,h) = app.main_window().inner_size_pixels();
    let offset = ((((h as f32)/2.0)/((w as f32)/2.0)).atan())/(2.0*PI);
    let angle_max = ((2.0*PI*turns as f32*((w as f32/2.0).pow(2.0)+(h as f32/2.0).pow(2.0)).sqrt())/(1000.0*(1.0-(width/turns as f32)).pow(cur_coefficient))).pow(1.0/cur_coefficient);
    let rad_max = (1.0/(2.0*PI*turns as f32))*angle_max.pow(cur_coefficient);

    Model {
        _window,
        state: AppState::MainMenu,
        last_frame_time: Instant::now(),
        fps_timer: Instant::now(),
        color1,
        color2,
        points: vec![],
        rot:0.0,
        speed:0.2,
        offset,
        count,
        turns,
        angle_max,
        rad_max,
        cur_coefficient,
        warp,
        x: vec![],
        y: vec![],
        w,
        h,
        width,
        egui,
        frame_count: 0,
        fps: 0.0,
        needs_update:true,
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);


    //listen for window resizing
    if let nannou::winit::event::WindowEvent::Resized(new_size) = event {
        model.w = new_size.width;
        model.h = new_size.height;
        model.needs_update = true;
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {

    let ctx = model.egui.begin_frame();
    
    match model.state {
        AppState::MainMenu => {
            
            let styles = vec![
                "Classic Spiral",
                "Placeholder 1",
                "Placeholder 2",
                "Placeholder 3",
                "Placeholder 4",
            ];
            
            egui::CentralPanel::default().show(&ctx, |ui| {
                ui.vertical_centered(|ui|{
                    ui.heading("Select Spiral");
                    ui.separator();
                    ui.horizontal_wrapped(|ui|{
                        for &label in &styles {
                            let clicked = ui
                                .add_sized([150.0,200.0], egui::Button::new(label))
                                .clicked();
                            if clicked {
                                model.state = AppState::SpiralView
                            }
                        }
                    });
                    ui.separator();
                })
            });
        }
        
        AppState::SpiralView => {
        let now = Instant::now();
        let delta_time = now.duration_since(model.last_frame_time).as_secs_f32();
        model.last_frame_time = now;

        model.rot += model.speed * delta_time;

        // FPS Counter
        model.frame_count += 1;
        let elapsed = now.duration_since(model.fps_timer);
        if elapsed >= Duration::from_secs(1) {
        model.fps = model.frame_count as f32 / elapsed.as_secs_f32();
        model.frame_count = 0;
        model.fps_timer = now;
        println !("FPS: {:.1}", model.fps);  // print once per second, very low overhead
        }


        // GUI Code
        egui::Window::new("Spiral Controls").show( & ctx, | ui | {
        model.needs_update |= ui.add(egui::Slider::new( & mut model.turns, 1..=15).text("Turns")).changed();
        ui.add(egui::Slider::new( & mut model.speed, - 5.0..=5.0).text("Speed"));
        ui.add(egui::Slider::new( & mut model.offset, 0.0..=1.0).text("Offset"));
        model.needs_update |= ui.add(egui::Slider::new( & mut model.warp, - 0.5..=0.5).text("Warp")).changed();
        ui.add(egui::Slider::new( & mut model.count, 90..=1080).text("Resolution"));
        ui.label("Color 1");
        ui.color_edit_button_srgb( & mut model.color1);
        ui.label("Color 2");
        ui.color_edit_button_srgb( & mut model.color2);
        });

        if model.needs_update {
        model.cur_coefficient = 10.0.powf(model.warp);
        model.angle_max = ((2.0 * PI * model.turns as f32 * ((model.w as f32/ 2.0).pow(2.0) + (model.h as f32 /2.0).pow(2.0)).sqrt()) / (1000.0 * (1.0 - (model.width / model.turns as f32)).pow(model.cur_coefficient))).pow(1.0 / model.cur_coefficient);
        model.rad_max = (1.0 /(2.0 * PI * model.turns as f32)) * model.angle_max.pow(model.cur_coefficient);

        model.needs_update = false;
        }


        // rotate the spiral and clear points cache from the previous frame
        model.x.clear();
        model.y.clear();


        // outer spiral edge points
        for i in 0..=model.count {
        let cur = i as f32 / model.count as f32;
        let cur_rot = 2.0 *PI * (cur * model.turns as f32 + model.offset + model.rot);
        let cur_rad = model.rad_max * cur.pow(model.cur_coefficient);
        model.x.push(cur_rad * cur_rot.cos());
        model.y.push(cur_rad * cur_rot.sin());
        }
        // inner spiral edge (reversed) points
        for i in (0..=(model.count as f32 * (1.0 - (model.width / model.turns as f32))) as i32).rev() {
        let cur = i as f32 / model.count as f32;
        let cur_rot = 2.0 * PI * (cur * (model.turns as f32) + model.offset + model.rot - model.width);
        let cur_rad = model.rad_max * cur.pow(model.cur_coefficient);
        model.x.push(cur_rad * cur_rot.cos());
        model.y.push(cur_rad * cur_rot.sin());
        }
        // collecting points into one vector
        model.points = model.x.iter()
        .zip(model.y.iter())
        .map( | ( & x, & y) | pt2(x * 1000.0, y * 1000.0))
        .collect();} }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let color1rgb = u83_to_rgb(model.color1);
    let color2rgb = u83_to_rgb(model.color2);

    draw.background().color(color1rgb);
    draw.polygon()
        .color(color2rgb)
        .points(model.points.iter().cloned());

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}


// converts 3 u8 integers into nannou rgb
fn u83_to_rgb(color: [u8;3]) -> Rgb {
    rgb(
        color[0] as f32 / 255.0,
        color[1] as f32 / 255.0,
        color[2] as f32 / 255.0,
    )
}