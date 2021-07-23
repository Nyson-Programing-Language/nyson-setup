use curl::easy::Easy;
use fltk::button::*;
use fltk::enums::*;
use fltk::frame::*;
use fltk::prelude::*;
use fltk::window::*;
use fltk::*;
use std::process::Command;
use std::{env, fs};
extern crate serde_json;

fn main() {
    let mut step = 0;
    let mut install_rust = false;
    let mut has_rust;
    match which::which("cargo") {
        Ok(_) => has_rust = true,
        Err(_) => has_rust = false,
    };
    let mut loc: String;
    if env::consts::OS == "windows" {
        let output = Command::new("echo")
            .args(["C:\\Users\\%username%"])
            .output()
            .expect("failed to execute process");
        loc = String::from_utf8_lossy(&output.stdout).to_string();
    } else {
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
            if install_rust == true {
                if env::consts::OS == "windows" {
                    Command::new("curl")
                        .args(["-L", "https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe", "--output", "rust.exe"])
                        .output()
                        .expect("failed to execute process");
                    Command::new("rust.exe")
                        .args([""])
                        .output()
                        .expect("failed to execute process");
                    fs::remove_file("rust.exe");
                } else {
                    Command::new("curl")
                        .args(["--proto", "=https", "--tlsv1.2", "https://sh.rustup.rs", "--output", "rustinstaller.sh"])
                        .output()
                        .expect("failed to execute process");
                    Command::new("chmod")
                        .args(["+x", "rustinstaller.sh"])
                        .output()
                        .expect("failed to execute process");
                    Command::new("sh")
                        .args(["-c", "./rustinstaller.sh -y"])
                        .output()
                        .expect("failed to execute process");
                    fs::remove_file("rustinstaller.sh");
                }
                has_rust = true;
                install_rust = false;
                step = 0;
                text.set_label("Finished installing rust");
            } else if has_rust == false {
                text.set_label("It does not look like you have\nrust do you want to install it?");
                install_rust = true;
                step = 0;
            } else {
                text.set_label("Where do you want nyson\ninstalled?");
                input.set_value(&loc);
                input.show();
            }
        }
        else if step == 2 {
            input.hide();
            loc = input.value();
            let output = Command::new("curl")
                .args(["-A", "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/81.0", "https://api.github.com/repos/Nyson-Programing-Language/nyson/releases/latest"])
                .output()
                .expect("failed to execute process");
            let json: serde_json::Value = serde_json::from_str(String::from_utf8_lossy(&output.stdout).to_string().as_str())
                .expect("file should be proper JSON");
            fs::create_dir(loc.clone());
            if env::consts::OS == "windows" {
                let url = json["zipball_url"].to_owned().to_string().replace("\"", "");
                Command::new("curl")
                    .args(["-L", url.as_str(), "--output", "nyson.zip"])
                    .output()
                    .expect("failed to execute process");
                Command::new("tar")
                    .args(["-xf", "nyson.zip", "-C", &loc, "--strip-components=1"])
                    .output()
                    .expect("failed to execute process");
                fs::remove_file("nyson.zip");
            } else {
                let url = json["tarball_url"].to_owned().to_string().replace("\"", "");
                Command::new("curl")
                    .args(["-L", url.as_str(), "--output", "nyson.tar.gz"])
                    .output()
                    .expect("failed to execute process");
                Command::new("tar")
                    .args(["-xf", "nyson.tar.gz", "-C", &loc, "--strip-components=1"])
                    .output()
                    .expect("failed to execute process");
                fs::remove_file("nyson.tar.gz");
            }
            let mut cargo = "".to_string();
            cargo.push_str(&loc);
            cargo.push_str("/Cargo.toml");
            Command::new("cargo")
                .args(["build", "--release", "--manifest-path", cargo.as_str()])
                .output()
                .expect("failed to execute process");
            if env::consts::OS == "windows" {
                let mut PATH = r"PATH=%PATH%;".to_string();
                PATH.push_str(&loc);
                PATH.push_str(r"/target/release");
                Command::new("SET")
                    .args([PATH.as_str()])
                    .output()
                    .expect("failed to execute process");
                Command::new("setx")
                    .args(["/M", "PATH", PATH.as_str()])
                    .output()
                    .expect("failed to execute process");
                text.set_label("Finished installing nyson");
            } else {
                let mut PATH = loc.clone();
                PATH.push_str(r"/target/release/nyson");
                Command::new("sudo")
                    .args(["-k", "cp", &PATH, "/usr/bin"])
                    .output()
                    .expect("failed to execute process");
                text.set_label("Finished installing nyson");
            }
        }
        else if step == 3 {
            std::process::exit(1);
        }
    });

    cancel.set_callback(move |_| {
        std::process::exit(1);
    });

    app.run().unwrap();
}

fn copy(path1: String, path2: String) -> std::io::Result<()> {
    fs::copy(path1, path2)?;
    Ok(())
}
