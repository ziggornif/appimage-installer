use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::{env, fs, io, process};

use clap::Parser;

static HOME: OnceLock<String> = OnceLock::new();

fn get_home_var() -> String {
    match env::var("HOME") {
        Ok(s) => s,
        Err(_) => "".to_string(),
    }
}

/// AppImage application installer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Application name
    #[arg(short, long)]
    name: Option<String>,

    /// AppImage file
    #[arg(short, long)]
    file: String,

    /// Application description
    #[arg(short, long)]
    description: Option<String>,

    /// Application icon
    #[arg(short, long)]
    icon: Option<String>,

    /// Application target location
    #[arg(short, long)]
    target: Option<String>,

    /// Application category
    #[arg(short, long)]
    category: Option<String>,
}

fn ask_user(sentence: String, mandatory: bool, default: Option<String>) -> String {
    println!("{sentence}");
    let mut response = String::new();

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut response)
        .expect("Fail to read user input");

    if mandatory && response.trim().is_empty() {
        ask_user(sentence, mandatory, default)
    } else if response.trim().is_empty() && default.is_some() {
        default.unwrap()
    } else {
        response.trim().to_owned()
    }
}

fn get_appname(name: Option<String>) -> String {
    match name {
        Some(name) => name,
        None => ask_user(
            "Enter the application name (ex: FreeCAD):".to_owned(),
            true,
            None,
        ),
    }
}

fn get_icon(icon: Option<String>) -> String {
    match icon {
        Some(name) => name,
        None => ask_user(
            "Enter the icon file path (ex: ./freecad.svg):".to_owned(),
            false,
            None,
        ),
    }
}

fn get_description(icon: Option<String>) -> String {
    match icon {
        Some(name) => name,
        None => ask_user("Enter the application description:".to_owned(), false, None),
    }
}

fn get_target(target: Option<String>) -> String {
    match target {
        Some(target) => target,
        None => {
            let default_target = Path::new(HOME.get().unwrap()).join("Apps");
            ask_user(
                "Enter the application target directory (default: $USER/Apps):".to_owned(),
                false,
                Some(default_target.to_str().unwrap().to_string()),
            )
        }
    }
}

fn get_category(category: Option<String>) -> String {
    match category {
        Some(name) => name,
        None => ask_user(
            "Enter the application category (ex: Graphics):".to_owned(),
            false,
            None,
        ),
    }
}

fn validate_file_ext(file: &str) -> bool {
    let extension = Path::new(file).extension().and_then(OsStr::to_str);
    match extension {
        Some(ext) => ext.to_lowercase() == "appimage",
        None => false,
    }
}

fn validate_src_icon(icon: &Path) -> bool {
    icon.exists()
}

fn move_appimage(file: &str, target: &PathBuf) {
    if target.exists() {
        let remove_answer = ask_user(
            "Application already installed, do you want to remove the existing AppImage file ? (y/n)"
                .to_owned(),
            true, None
        );

        if remove_answer == *"y" {
            std::fs::remove_file(target).expect("Error while removing existing AppImage file");
        } else {
            println!("Abort installation. Bye.");
            process::exit(0)
        }
    }

    match fs::copy(file, target) {
        Ok(_) => println!("Application installed in {} directory", target.display()),
        Err(err) => {
            eprintln!("Fail to install application: {}", err);
            process::exit(1);
        }
    }
}

fn copy_icon(src: &str, target: &PathBuf) {
    match fs::copy(src, target) {
        Ok(_) => println!("Icon has been copied in {} directory", target.display()),
        Err(err) => eprintln!("Fail to install application: {}", err),
    }
}

fn generate_desktop_file(name: &str, desc: &str, icon: &str, category: &str, target: &str) {
    let content = format!(
        r#"
[Desktop Entry]
Type=Application
Name={name}
Exec={target}
Icon={icon}
Comment={desc}
Terminal=false
Categories={category};

TryExec={target}
PrefersNonDefaultGPU=false
        "#
    );

    let mut desktop_file = File::create_new(format!(
        "{}/.local/share/applications/{}.desktop",
        &HOME.get().unwrap(),
        name
    ))
    .unwrap();
    desktop_file.write_all(content.as_bytes()).unwrap();
}

fn main() {
    let home_var = HOME.get_or_init(get_home_var);
    let args = Args::parse();
    println!("Welcome to AppImage desktop installer");

    if !validate_file_ext(&args.file) {
        println!("Invalid AppImage file, abort installation.");
        process::exit(1);
    }

    let app_name = get_appname(args.name);
    let app_desc = get_description(args.description);
    let mut app_icon = get_icon(args.icon);
    let app_target = get_target(args.target);
    let app_category = get_category(args.category);

    if !app_icon.is_empty() {
        let src_path = Path::new(&app_icon);
        let filename = src_path.file_name().unwrap();
        let target_icon = Path::new(home_var)
            .join(".local/share/icons")
            .join(filename);
        if !validate_src_icon(src_path) {
            println!("Invalid icon file, abort installation.");
            process::exit(1);
        }
        copy_icon(&app_icon, &target_icon);
        app_icon = target_icon.to_str().unwrap().to_string();
    }

    let src_path = Path::new(&args.file);
    let filename = src_path.file_name().unwrap();
    let target_dir = Path::new(&app_target);
    let target_appimg_path = target_dir.join(filename);
    move_appimage(&args.file, &target_appimg_path);

    generate_desktop_file(
        &app_name,
        &app_desc,
        &app_icon,
        &app_category,
        target_appimg_path.to_str().unwrap(),
    );
}
