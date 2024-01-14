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

// fn read_file(_path: String) {
//     let file = File::open(_path).unwrap();
//     let mut buf_reader = BufReader::new(file);
//     let mut contents = String::new();
//     buf_reader.read_to_string(&mut contents).unwrap();
// }

fn main() -> Result<(), slint::PlatformError> {
    // --!Variables--
    const EDKEY: &str = "magickey";
    const BINPATH: &str = "data-bin/"; //replace to bin for production-code
    
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

    // --!Load--
    let account_file: &String = &format!("{}account.txt", BINPATH);
    let crypt_file: &String = &format!("{}crypt.txt", BINPATH);
    let account_setup: bool = Path::new(account_file).exists() && Path::new(crypt_file).exists();
    let render_msg: String;
    let current_user: String;
    if !account_setup {
        File::create(account_file).unwrap();
        File::create(crypt_file).unwrap();
        render_msg = format!("Welcome, files are build");
        current_user = format!("");
    }
    else {
        render_msg = format!("Welcome back");
        current_user = format!("Jace");
    }
    let ui_handle:Weak<AppWindow> = ui.as_weak();
    let ui_init: AppWindow = ui_handle.unwrap();
    ui_init.set_read_items(render_msg.into());
    ui_init.set_user_name(current_user.into());

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

        // ?clean values
        // todo json clean
        let cleaned_ref: &str = _ref.trim();
        let cleaned_pw: &str = _pw.trim();
        let cleaned_descr: &str = _descr.trim();

        // ?check if valid inputs
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

        let ui: AppWindow = ui_handle.unwrap();
        if has_error {
            // ?return early
            let res: String = format!("Error: \n{}", error.join("\n").trim());
            ui.set_feedback_out(res.into());
        }
        else {
            // ?encrypt
            let crypted_pw: String = encryptor.encrypt_str_to_base64(cleaned_pw);
            let decrypted_pw: String = encryptor.decrypt_base64_to_string(&crypted_pw).unwrap();

            let afile= &format!("{}crypts.txt", BINPATH);
            // ?read file
            let contentfile = File::open(afile).unwrap();
            let mut buf_reader = BufReader::new(contentfile);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();

            // ?write to file
            contents.push_str(format!(
                                "\n{{ 'ref': '{}', 'crypt': '{}', 'descr': '{}' }}", 
                                cleaned_ref, crypted_pw, cleaned_descr
                            ).as_str());
            let mut file = File::create(afile).unwrap();
            file.write_all(contents.as_bytes()).unwrap();

            // ?user feedback
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
