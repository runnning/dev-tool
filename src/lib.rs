pub mod logic;
pub mod ui;
pub mod utils;

slint::slint! {
    export { MainWindow } from "src/ui/main.slint";
}
