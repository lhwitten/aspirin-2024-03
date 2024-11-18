mod cffi;
mod gui;
mod serial;

use gui::*;
use serial::*;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<(), eframe::Error> {
    let circle_state = Arc::new(Mutex::new(CircleState::new()));

    // Start the serial thread
    let state_clone = circle_state.clone();
    thread::spawn(move || unsafe {
        match init_serial() {
            Ok(port_handle) => loop {
                if let Some(controller_state) = read_controller_state(port_handle) {
                    let mut state = state_clone.lock().unwrap();
                    state.update(controller_state.buttons, controller_state.adc_value);
                }
            },
            Err(err) => {
                eprintln!("Failed to initialize serial port: {}", err);
            }
        }
    });

    // Start the GUI
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Circle Controller",
        options,
        Box::new(|_cc| Ok(Box::new(App::initialize(circle_state.clone())))),
    )
}
