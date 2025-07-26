mod visuals;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use std::time::{Instant, Duration};
use crate::visuals::{ClassicVisual, Visual};

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
    speed: f32, // stores the rotation speed of the spiral
    needs_update: bool, // tracks whether the spiral generation values need updating this frame
    
    active_visual: Box<dyn Visual>,
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
    let (w,h) = app.main_window().inner_size_pixels();
    Model {
        _window,
        state: AppState::MainMenu,
        last_frame_time: Instant::now(),
        fps_timer: Instant::now(),
        color1,
        color2,
        speed:0.2,
        w,
        h,
        egui,
        frame_count: 0,
        fps: 0.0,
        needs_update:true,
        active_visual: Box::new(ClassicVisual::new(w,h)),
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);


    //listen for window resizing
    if let nannou::winit::event::WindowEvent::Resized(new_size) = event {
        model.w = new_size.width;
        model.h = new_size.height;
        model.active_visual.resize(model.w,model.h);
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
                ui.vertical_centered(|ui| {
                    ui.heading("Select Spiral");
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        for &label in &styles {
                            let clicked = ui
                                .add_sized([150.0, 200.0], egui::Button::new(label))
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

            // FPS Counter
            model.frame_count += 1;
            let elapsed = now.duration_since(model.fps_timer);
            if elapsed >= Duration::from_secs(1) {
                model.fps = model.frame_count as f32 / elapsed.as_secs_f32();
                model.frame_count = 0;
                model.fps_timer = now;
                println!("FPS: {:.1}", model.fps);
            }
            
            let active_visual = &mut model.active_visual;

            if model.needs_update {
                active_visual.setup(model.w, model.h, model.speed, model.needs_update, model.color1, model.color2);
                model.needs_update = false;
            }
            
            active_visual.update(delta_time);
            
            egui::Window::new("Spiral Controls").show(&ctx, |ui| {
                active_visual.ui(ui);
            });
            
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.active_visual.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
    