use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    
    ui.on_close_feedback(move || {
        let ui: AppWindow = ui_handle.unwrap();
        ui.set_feedback_out("feedback reset".into());
        ui.set_is_open(false.into());
    });
    
    let ui_handle = ui.as_weak();
    ui.on_set_feedback(move |_msg: SharedString| {
        let ui: AppWindow = ui_handle.unwrap();
        ui.set_feedback_out(_msg.into());
        ui.set_is_open(true.into());
    });

    ui.run()
}
