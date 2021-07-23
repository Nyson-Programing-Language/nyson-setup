use fltk::*;
use std::{env, fs};
use std::path::{Path, PathBuf};
use fltk::window::*;
use fltk::prelude::*;
use fltk::button::*;
use fltk::tree::*;
use fltk::frame::*;
use fltk::enums::*;
use fltk::input::*;
use which::which;
use std::process::Command;

fn main() {
    let mut step = 0;
    let mut has_git;
    let mut install_git = false;
    let mut install_rust = false;
    match which::which("git") {
        Ok(_) => {has_git = true}
        Err(_) => {has_git = false}
    };
    let mut has_rust;
    match which::which("cargo") {
        Ok(_) => {has_rust = true}
        Err(_) => {has_rust = false}
    };
    let mut loc:String;
    if env::consts::OS == "windows" {
        let output = Command::new("echo")
            .args(["C:\\Users\\%username%"])
            .output()
            .expect("failed to execute process");
        loc = String::from_utf8_lossy(&output.stdout).to_string();
    }
    else {
        let output = Command::new("whoami")
            .output()
            .expect("failed to execute process");
        loc = "/home/".to_string();
        loc.push_str(String::from_utf8_lossy(&output.stdout).to_string().as_str());
        loc.pop();
    }
    loc.push_str("/nyson");
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default()
        .with_label("Nyson Installer")
        .with_size(400, 300)
        .center_screen();
    let mut ok = Button::new(240, 255, 80, 40, "OK");
    let mut cancel = Button::new(80, 255, 80, 40, "CANCEL");
    let mut text = Frame::new(0, 0, 400, 200, "");
    text.set_label_size(20);
    text.set_label_color(Color::White);
    let mut input = input::Input::new(50, 150, 300, 50, "");
    input.hide();
    //wind.make_resizable(true);
    wind.show();
    text.set_label("This is the nyson installer\ndo you want to continue?");
    
    ok.set_callback(move |_| {
        step = step + 1;
        if step == 1 {
            if install_git == true {
                text.set_label("Installing git");
                //installs git
                has_git = true;
                install_git = false;
                step = 0;
            }
            else if install_rust == true {
                text.set_label("Installing rust");
                //installs rust
                has_rust = true;
                install_rust = false;
                step = 0;
            }
            else if has_git == false {
                text.set_label("It does not look like you have\ngit do you want to install it?");
                install_git = true;
                step = 0;
            }
            else if has_rust == false {
                text.set_label("It does not look like you have\nrust do you want to install it?");
                install_rust = true;
                step = 0;
            }
            else {
                text.set_label("Where do you want nyson\ninstalled?");
                input.set_value(&loc);
                input.show();
            }
        }
    });

    cancel.set_callback(move |_| {
        std::process::exit(1);
    });
    
    app.run().unwrap();
}