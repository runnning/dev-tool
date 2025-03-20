use slint::*;

mod modules;
use modules::*;

include_modules!();

fn main() -> Result<(), PlatformError> {
    let main_window = MainWindow::new()?;

    // 获取当前时间戳
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

    let main_window_weak = main_window.as_weak();
    main_window.on_format_json(move || {
        let main_window = main_window_weak.unwrap();
        let input = main_window.get_json_input();
        main_window.set_json_output(format_json(&input).into());
    });

    main_window.run()
}