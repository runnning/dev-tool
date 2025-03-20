use slint::ComponentHandle;
use crate::{MainWindow, modules::*};

pub struct TimestampHandler;

impl TimestampHandler {
    pub fn setup(main_window: &MainWindow) {
        let main_window_weak = main_window.as_weak();
        main_window.on_get_current_timestamp(move || {
            let main_window = main_window_weak.unwrap();
            main_window.set_timestamp_input(get_current_timestamp().into());
        });

        let main_window_weak = main_window.as_weak();
        main_window.on_get_current_ms_timestamp(move || {
            let main_window = main_window_weak.unwrap();
            main_window.set_timestamp_input(get_current_ms_timestamp().into());
        });

        let main_window_weak = main_window.as_weak();
        main_window.on_convert_timestamp(move || {
            let main_window = main_window_weak.unwrap();
            let input = main_window.get_timestamp_input();
            main_window.set_timestamp_output(convert_timestamp(&input).into());
        });

        let main_window_weak = main_window.as_weak();
        main_window.on_convert_ms_timestamp(move || {
            let main_window = main_window_weak.unwrap();
            let input = main_window.get_timestamp_input();
            main_window.set_timestamp_output(convert_ms_timestamp(&input).into());
        });
    }
} 