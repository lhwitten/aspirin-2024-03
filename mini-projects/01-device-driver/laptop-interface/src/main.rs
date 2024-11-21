mod cffi;
mod gui;
mod serial;

use cffi::*;
use gui::*;
use serial::*;
use std::io::{self};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), eframe::Error> {
    println!("Do you want to run the onboarding project? (y/n):");

    // Get the user's input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim().to_lowercase();

    if input == "y" {
        // Run the onboarding project: connect to two controllers
        println!("Starting the onboarding project...");
        run_onboarding_project();
    } else {
        // Run the GUI for a single controller
        println!("Starting the GUI with a single controller...");

        let circle_state = Arc::new(Mutex::new(CircleState::new()));

        // Get available ports
        let port_name = unsafe {
            match get_ports() {
                Ok(mut ports) if !ports.is_empty() => ports.remove(0),
                Ok(_) => {
                    eprintln!("No USB controllers found for GUI.");
                    return Ok(());
                }
                Err(err) => {
                    eprintln!("Failed to list ports: {}", err);
                    return Ok(());
                }
            }
        };

        // Start the serial thread for the single controller
        let state_clone = circle_state.clone();
        thread::spawn(move || unsafe {
            match init_serial(&port_name) {
                Ok(port_handle) => {
                    blocking_write(port_handle, b"start controller\n").ok();
                    loop {
                        if let Some(controller_state) = read_controller_state(port_handle) {
                            let mut state = state_clone.lock().unwrap();
                            state.update(controller_state.buttons, controller_state.adc_value);
                        }
                    }
                }
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
        )?;
    }

    Ok(())
}

fn run_onboarding_project() {
    unsafe {
        match get_ports() {
            Ok(ports) if ports.len() >= 2 => {
                let port1 = ports[0].clone();
                let port2 = ports[1].clone();

                // Channels to send game commands to controller threads
                let (tx1, rx1) = channel();
                let (tx2, rx2) = channel();

                // Thread for Controller 0
                let controller1_thread = thread::spawn(move || match init_serial(&port1) {
                    Ok(port_handle) => {
                        println!("Controller 0 Initialized.");
                        run_controller_loop(0, port_handle, rx1);
                        sp_close(port_handle);
                        sp_free_port(port_handle);
                    }
                    Err(err) => {
                        eprintln!("Failed to initialize Controller 0: {}", err);
                    }
                });

                // Thread for Controller 1
                let controller2_thread = thread::spawn(move || match init_serial(&port2) {
                    Ok(port_handle) => {
                        println!("Controller 1 Initialized.");
                        run_controller_loop(1, port_handle, rx2);
                        sp_close(port_handle);
                        sp_free_port(port_handle);
                    }
                    Err(err) => {
                        eprintln!("Failed to initialize Controller 1: {}", err);
                    }
                });

                // Main thread handles user input
                loop {
                    println!("Get the highest score by pressing the buttons! Type 'start' to begin the game or 'exit' to quit:");
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    let input = input.trim().to_lowercase();

                    match input.as_str() {
                        "start" => {
                            tx1.send(GameCommand::Start)
                                .expect("Failed to send start command to Controller 0");
                            tx2.send(GameCommand::Start)
                                .expect("Failed to send start command to Controller 1");
                        }
                        "exit" => {
                            tx1.send(GameCommand::Exit)
                                .expect("Failed to send exit command to Controller 0");
                            tx2.send(GameCommand::Exit)
                                .expect("Failed to send exit command to Controller 1");
                            break;
                        }
                        _ => {
                            println!("Unknown command. Please type 'start' or 'exit'.");
                        }
                    }
                }

                // Wait for threads to finish
                let _ = controller1_thread.join();
                let _ = controller2_thread.join();
            }
            Ok(_) => eprintln!("Not enough USB controllers found."),
            Err(err) => eprintln!("Failed to list ports: {}", err),
        }
    }
}

enum GameCommand {
    Start,
    Exit,
}

fn run_controller_loop(
    player_id: usize,
    port_handle: *mut cffi::SpPort,
    rx: Receiver<GameCommand>,
) {
    loop {
        match rx.recv() {
            Ok(GameCommand::Start) => {
                println!("Player {} -> Starting game!", player_id);

                // Start countdown
                unsafe {
                    serial::blocking_write(port_handle, b"set ready led\n").ok();
                }
                thread::sleep(Duration::from_secs(1));
                unsafe {
                    serial::blocking_write(port_handle, b"set set led\n").ok();
                }
                thread::sleep(Duration::from_secs(1));
                unsafe {
                    serial::blocking_write(port_handle, b"set go led\n").ok();
                }
                thread::sleep(Duration::from_secs(1));

                println!("Player {} -> GO!", player_id);
                unsafe {
                    serial::blocking_write(port_handle, b"set all leds\n").ok();
                }
                unsafe {
                    serial::blocking_write(port_handle, b"start controller\n").ok();
                }

                // Play game for 5 seconds
                let mut score = 0;
                let game_duration = Duration::from_secs(5);
                let start_time = std::time::Instant::now();

                while start_time.elapsed() < game_duration {
                    if let Some(state) = unsafe { serial::read_controller_state(port_handle) } {
                        if state.buttons != 0 {
                            score += state.buttons as usize;
                            println!(
                                "Player {} -> Current Score: {}, Button Value: {:08b}",
                                player_id, score, state.buttons
                            );
                        }
                    }
                    thread::sleep(Duration::from_millis(100));
                }

                // End game round
                println!("\n<5 seconds over>");
                println!("Player {} -> Final Score: {}", player_id, score);

                // Reset LEDs
                unsafe {
                    serial::blocking_write(port_handle, b"clear all leds\n").ok();
                }
            }
            Ok(GameCommand::Exit) => {
                println!("Player {} -> Exiting game.", player_id);
                unsafe {
                    serial::blocking_write(port_handle, b"reset\n").ok();
                }
                break;
            }
            Err(_) => {
                eprintln!("Player {} -> Command channel closed.", player_id);
                break;
            }
        }
    }
}
