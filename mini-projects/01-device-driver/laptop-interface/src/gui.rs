use eframe::egui;
use std::sync::{Arc, Mutex};

/// Represents the state of the circle in the GUI
pub struct CircleState {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

pub fn normalize_adc_value(adc_value: u16, step: u16) -> u16 {
    (adc_value + step / 2) / step * step
}

impl CircleState {
    pub fn new() -> Self {
        Self {
            x: 200.0,
            y: 200.0,
            radius: 50.0,
        }
    }

    /// Updates the circle's position and size based on input
    pub fn update(&mut self, buttons: u8, adc_value: u16) {
        let step = 30.0;
        if buttons & 0b10000000 != 0 {
            self.y -= step;
            self.x += step;
        } // NE
        if buttons & 0b01000000 != 0 {
            self.y += step;
            self.x += step;
        } // SE
        if buttons & 0b00100000 != 0 {
            self.y += step;
            self.x -= step;
        } // SW
        if buttons & 0b00010000 != 0 {
            self.y -= step;
            self.x -= step;
        } // NW
        if buttons & 0b00001000 != 0 {
            self.x += step;
        } // Right
        if buttons & 0b00000100 != 0 {
            self.x -= step;
        } // Left
        if buttons & 0b00000010 != 0 {
            self.y += step;
        } // Bottom
        if buttons & 0b00000001 != 0 {
            self.y -= step;
        } // Top

        // Adjust circle size with rounded ADC value
        let rounded_adc = normalize_adc_value(adc_value, 30); // Round to nearest 10
        self.radius = (rounded_adc as f32) / 10.0;
    }
}

/// The main application struct for `eframe`
pub struct App {
    pub circle_state: Arc<Mutex<CircleState>>,
}

impl App {
    /// Initializes the application
    pub fn initialize(circle_state: Arc<Mutex<CircleState>>) -> Self {
        Self { circle_state }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let circle = self.circle_state.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Use the buttons to move the circle and adjust size with ADC input.");
            ui.add_space(20.0);
            ui.painter().circle_filled(
                egui::pos2(circle.x, circle.y),
                circle.radius,
                egui::Color32::from_rgb(200, 50, 50),
            );
        });

        ctx.request_repaint(); // Repaint to reflect state updates
    }
}
