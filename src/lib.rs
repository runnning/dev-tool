pub mod ui;
pub mod logic;
mod utils;

slint::slint! {
    export { MainWindow } from "src/ui/main.slint";
}

pub use logic::time::TimeLogic;
pub use logic::event::EventHandler;
pub use logic::json::JsonLogic;
pub use utils::time;
pub use utils::json; 