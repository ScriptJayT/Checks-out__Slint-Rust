use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

#[macro_use] extern crate magic_crypt;
use magic_crypt::{MagicCryptTrait, MagicCrypt256};
use passwords::PasswordGenerator;

use slint::SharedString;
use slint::Weak;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // --!Variables--
    const EDKEY: &str = "magickey";
    const BINPATH: &str = "data-bin/";
    
    let ui: AppWindow = AppWindow::new()?;
    let pg: PasswordGenerator = PasswordGenerator {
        length: 12,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };
    let encryptor:MagicCrypt256 = new_magic_crypt!(EDKEY, 256);

    // --!Render--
    let account_file: &String = &format!("{}foo.txt", BINPATH);
    let account_setup: bool = Path::new(account_file).exists();

    let render_msg: String;
    if account_setup {
        let file = File::open(account_file).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        render_msg = format!("reading: {}", contents);
    }
    else {
        let mut file = File::create(account_file).unwrap();
        file.write_all(b"Hello, world!").unwrap();

        render_msg = format!("setup");
    }

    // --!Load--
    let ui_handle:Weak<AppWindow> = ui.as_weak();
    let ui_init: AppWindow = ui_handle.unwrap();
    ui_init.set_read_items(render_msg.into());

    // --!Events--
    // ?Close window
    ui.on_close_window(move || {
        std::process::exit(200);
    });
    // ?User feedback
    let ui_handle:Weak<AppWindow> = ui.as_weak();
    ui.on_close_feedback(move || {
        let ui: AppWindow = ui_handle.unwrap();
        ui.set_feedback_out("feedback reset".into());
        ui.set_is_open(false.into());
    });
    let ui_handle:Weak<AppWindow> = ui.as_weak();
    ui.on_set_feedback(move |_msg: SharedString| {
        let ui: AppWindow = ui_handle.unwrap();
        ui.set_feedback_out(_msg.into());
        ui.set_is_open(true.into());
    });
    // ?Passwords
    // generate
    let ui_handle:Weak<AppWindow> = ui.as_weak();
    ui.on_generate_password(move || {
        let ui: AppWindow = ui_handle.unwrap();
        let new_pw: String = pg.generate_one().unwrap();
        ui.set_suggested_password(new_pw.into());
    });
    // save & encrypt
    // todo: save to file
    let ui_handle:Weak<AppWindow> = ui.as_weak();
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
        let crypted_pw: String = encryptor.encrypt_str_to_base64(cleaned_pw);
        let decrypted_pw: String = encryptor.decrypt_base64_to_string(&crypted_pw).unwrap();

        //? return & reply
        let ui: AppWindow = ui_handle.unwrap();
        if has_error {
            let res: String = format!("Error: \n{}", error.join("\n").trim());
            ui.set_feedback_out(res.into());
        }
        else {
            let debug: String = format!(
                                    "\n\n\n--Debug--\nOriginal: {}\nCrypted: {}\nDecrypted: {}", 
                                    cleaned_pw, crypted_pw, decrypted_pw 
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
