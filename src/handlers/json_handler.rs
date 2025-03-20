use slint::ComponentHandle;
use crate::{MainWindow, modules::*};

pub struct JsonHandler;

impl JsonHandler {
    pub fn setup(main_window: &MainWindow) {
        let main_window_weak = main_window.as_weak();
        main_window.on_format_json(move || {
            let main_window = main_window_weak.unwrap();
            let input = main_window.get_json_input();
            main_window.set_json_output(format_json(&input).into());
        });
    }
} 