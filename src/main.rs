use slint::*;

mod modules;
mod handlers;

use handlers::*;

include_modules!();

fn main() -> Result<(), PlatformError> {
    let main_window = MainWindow::new()?;

    // 设置事件处理器
    TimestampHandler::setup(&main_window);
    JsonHandler::setup(&main_window);

    main_window.run()
}