use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // user feedback
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
    
    // save & encrypt passwords
    let ui_handle = ui.as_weak();
    ui.on_save_password(move |_ref: SharedString, _pw: SharedString, _descr: SharedString| {
        let mut has_error: bool = false;
        let mut error: [_; 2] = ["", ""];
        
        //? clean values
        let cleaned_ref: &str = _ref.trim();
        let cleaned_pw: &str = _pw.trim();
        let cleaned_descr: &str = _descr.trim();

        //? check if valid inputs
        let valid_ref: bool = cleaned_ref.chars().count() > 1;
        let valid_pw: bool = cleaned_pw.chars().count() > 1;
        if !valid_ref  {
            has_error = true;
            error[0] = " - `Reference` needs to be filled in";
        }
        if !valid_pw  {
            has_error = true;
            error[1] = " - `Password` needs to be filled in";
        }
        
        //? encrypt
        let crypted_pw: &str = cleaned_pw;

        //? return & reply
        let ui: AppWindow = ui_handle.unwrap();
        if has_error {
            let res: String = format!("Error: \n{}", error.join("\n").trim());
            ui.set_feedback_out(res.into());
        }
        else {
            let debug: String = format!(
                                    "\n\n\n--Debug--\nOriginal: {}\nCrypted: {}", 
                                    cleaned_pw, crypted_pw 
                                );

            let res: String = format!(
                                "Password saved for: `{}`\nwith:\n{}{}", 
                                cleaned_ref, cleaned_descr, debug
                            );
            ui.set_feedback_out(res.into());
        }
        
        ui.set_is_open(true.into());
    });

    ui.run()
}
