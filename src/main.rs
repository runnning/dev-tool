#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dev_tool::{MainWindow, logic::event::EventHandler};
use slint::ComponentHandle;

fn main() {
    let main_window = MainWindow::new().unwrap();
    
    // 创建事件处理器
    let event_handler = EventHandler::new(main_window.as_weak());
    
    // 注册事件处理函数
    event_handler.handle_events();
    
    // 运行应用程序
    main_window.run().unwrap();
}