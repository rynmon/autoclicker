#![windows_subsystem = "windows"]

use eframe::egui;
use enigo::{Enigo, Button, Direction, Settings, Mouse};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[cfg(windows)]
const ICON_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/icon.png"));

#[cfg(not(windows))]
const ICON_DATA: &[u8] = &[];

struct AutoClickerApp {
    clicking: Arc<Mutex<bool>>,
    cps: Arc<Mutex<u32>>, // clicks per second
    ctx: Option<egui::Context>,
}

impl Default for AutoClickerApp {
    fn default() -> Self {
        let clicking = Arc::new(Mutex::new(false));
        let clicking_clone = Arc::clone(&clicking);
        
        // Spawn background thread to monitor F6 key globally
        std::thread::spawn(move || {
            let device_state = DeviceState::new();
            let mut last_f6_state = false;
            
            loop {
                let keys = device_state.get_keys();
                let f6_pressed = keys.contains(&Keycode::F6);
                
                // Detect F6 key press (on key down, not while held)
                if f6_pressed && !last_f6_state {
                    // Stop clicking when F6 is pressed
                    let mut clicking_state = clicking_clone.lock().unwrap();
                    *clicking_state = false;
                }
                last_f6_state = f6_pressed;
                
                // Small delay to avoid busy-waiting
                std::thread::sleep(Duration::from_millis(50));
            }
        });
        
        Self {
            clicking,
            cps: Arc::new(Mutex::new(10)),
            ctx: None,
        }
    }
}

impl eframe::App for AutoClickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Store context for repaint requests from background thread
        if self.ctx.is_none() {
            self.ctx = Some(ctx.clone());
        }
        
        // Request repaint periodically to update UI if F6 was pressed
        ctx.request_repaint_after(Duration::from_millis(100));
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Auto Clicker");
            ui.separator();
            
            let clicking = *self.clicking.lock().unwrap();
            let cps = *self.cps.lock().unwrap();
            
            // Status display
            ui.horizontal(|ui| {
                ui.label("Status:");
                if clicking {
                    ui.colored_label(egui::Color32::GREEN, "ACTIVE");
                } else {
                    ui.colored_label(egui::Color32::GRAY, "INACTIVE");
                }
            });
            
            ui.add_space(10.0);
            
            // CPS control
            ui.horizontal(|ui| {
                ui.label("Clicks per second:");
                ui.add(egui::Slider::new(&mut *self.cps.lock().unwrap(), 1..=100));
                ui.label(format!("{}", cps));
            });
            
            ui.add_space(20.0);
            
            // Toggle button
            let button_text = if clicking { "Stop Clicking" } else { "Start Clicking" };
            let button_color = if clicking {
                egui::Color32::from_rgb(200, 50, 50)
            } else {
                egui::Color32::from_rgb(50, 200, 50)
            };
            
            if ui.add(egui::Button::new(button_text).fill(button_color)).clicked() {
                let mut clicking_state = self.clicking.lock().unwrap();
                *clicking_state = !*clicking_state;
                
                if *clicking_state {
                    // Start clicking task
                    let clicking_clone = Arc::clone(&self.clicking);
                    let cps_clone = Arc::clone(&self.cps);
                    
                    ctx.request_repaint();
                    
                    std::thread::spawn(move || {
                        // Wait 1 second before starting to click (gives user time to move mouse away)
                        std::thread::sleep(Duration::from_secs(1));
                        
                        // Check if still supposed to be clicking after delay
                        if !*clicking_clone.lock().unwrap() {
                            return;
                        }
                        
                        let settings = Settings::default();
                        let mut enigo = match Enigo::new(&settings) {
                            Ok(e) => e,
                            Err(_) => return, // If we can't create Enigo, just exit the thread
                        };
                        
                        loop {
                            if !*clicking_clone.lock().unwrap() {
                                break;
                            }
                            
                            // Get current CPS and calculate interval
                            let current_cps = *cps_clone.lock().unwrap();
                            let interval = Duration::from_secs_f64(1.0 / current_cps as f64);
                            
                            // Perform left mouse button click (press and release)
                            let _ = enigo.button(Button::Left, Direction::Press);
                            std::thread::sleep(Duration::from_millis(10)); // Small delay between press and release
                            let _ = enigo.button(Button::Left, Direction::Release);
                            
                            std::thread::sleep(interval);
                        }
                    });
                }
            }
            
            ui.add_space(20.0);
            
            // Instructions
            ui.separator();
            ui.label("Instructions:");
            ui.label("• Set your desired clicks per second");
            ui.label("• Click 'Start Clicking' to begin");
            ui.label("• Click 'Stop Clicking' to pause");
            ui.label("• Press F6 to stop clicking (works globally)");
            ui.label("• The clicker will click at the current mouse position");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    // Load embedded icon for window
    let icon = if !ICON_DATA.is_empty() {
        eframe::icon_data::from_png_bytes(ICON_DATA).ok()
    } else {
        None
    };
    
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([350.0, 320.0])
        .with_resizable(true);
    
    if let Some(icon_data) = icon {
        viewport = viewport.with_icon(icon_data);
    }
    
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    
    eframe::run_native(
        "Auto Clicker",
        options,
        Box::new(|_cc| Box::new(AutoClickerApp::default())),
    )
}
