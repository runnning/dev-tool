#![windows_subsystem = "windows"]

mod handlers;
mod modules;

use handlers::*;
use slint::{include_modules, PlatformError};

include_modules!();

fn main() -> Result<(), PlatformError> {
    let main_window = MainWindow::new()?;

    // 设置事件处理器
    TimestampHandler::setup(&main_window);
    JsonHandler::setup(&main_window);

    main_window.run()
}
