use slint::SharedString;
use slint::Weak;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // --!Variables--
    let ui: AppWindow = AppWindow::new()?;
    // let account_file = "account.txt";
    // let crypt_file = "crypt.txt";

    // --!Load--
    let ui_handle:Weak<AppWindow> = ui.as_weak();
    let ui_init: AppWindow = ui_handle.unwrap();
    ui_init.set_user_name("Jace".into());

    ui_init.set_feedback_out("Loaded".into());
    ui_init.set_is_open(true.into());

    // --!Events--
    // ?Close window
    ui.on_close_window(move || {
        std::process::exit(200);
    });
    // ?User feedback
    let ui_handle:Weak<AppWindow> = ui.as_weak();
    ui.on_close_feedback(move || {
        let ui: AppWindow = ui_handle.unwrap();
        ui.set_feedback_out("Closing...".into());
        slint::Timer::single_shot(std::time::Duration::from_millis(500), move || {
            ui.set_is_open(false.into());
        });
    });
    let ui_handle:Weak<AppWindow> = ui.as_weak();
    ui.on_set_feedback(move |_msg: SharedString| {
        let ui: AppWindow = ui_handle.unwrap();
        ui.set_feedback_out(_msg.into());
        ui.set_is_open(true.into());
    });

    ui.run()
}
