#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dev_tool::{MainWindow, logic::event::EventHandler};
use slint::ComponentHandle;

fn main() {
    let window = MainWindow::new().unwrap();
    let event_handler = EventHandler::new(window.as_weak());
    event_handler.init();
    window.run().unwrap();
}