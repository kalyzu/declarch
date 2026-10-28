// use serde::Deserialize;
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::path::Path;
use toml_edit::{DocumentMut, Value};

use crate::ConfigFile::{Aur, Flatpak, Pacman};

mod insert;
mod shell;

// #[derive(Deserialize, Debug)]
// struct Config {
//     pacman: PkgList,
//     aur: PkgList,
//     flatpak: PkgList,
// }
//
// #[derive(Deserialize, Debug)]
// struct PkgList {
//     packages: Vec<String>,
// }

pub enum ConfigFile {
    Pacman,
    Aur,
    Flatpak,
    LockFile,
}

impl ConfigFile {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Pacman => "pacman",
            Self::Aur => "aur",
            Self::Flatpak => "flatpak",
            Self::LockFile => "declarch",
        }
    }
}

pub fn get_config_file_path(config_file: &ConfigFile) -> String {
    let config_file_str = config_file.as_str();
    let file_path = match config_file {
        ConfigFile::LockFile => format!("test-data/{}.lock", config_file_str),
        _ => format!("test-data/{}.toml", config_file_str),
    };
    if let Err(e) = ensure_file_exists(&file_path) {
        eprintln!("{e}");
    }
    file_path
}

pub fn get_doc(path: &str) -> DocumentMut {
    if let Err(e) = ensure_file_exists(path) {
        eprintln!("{e}");
    }

    let toml_data = fs::read_to_string(path).expect(format!("{} not found", path).as_str());
    toml_data
        .parse::<DocumentMut>()
        .expect(format!("{} not found", path).as_str())
}

// pub fn run() -> Result<(), Box<dyn std::error::Error>> {
//     let conf_path = "test-data/config.toml";
//     let toml_data = fs::read_to_string(conf_path)?;
//     let config: Config = toml::from_str(&toml_data)?;
//
//     println!("{:?}", config.packages);
//     Ok(())
// }

// cred: gemini 3
fn toml_array_to_vec(arr: &toml_edit::Array) -> Vec<String> {
    arr.iter()
        .filter_map(|value| value.as_str()) // Converts &Value to Option<&str>
        .map(|s| s.to_string()) // Converts &str to String
        .collect() // Collects into Vec<String>
}

fn get_cfg_files_string() -> String {
    let mut files: Vec<String> = Vec::new();
    let path = get_config_file_path(&ConfigFile::Pacman);
    files.push(fs::read_to_string(&path).expect(format!("{} not found", path).as_str()));
    let path = get_config_file_path(&ConfigFile::Aur);
    files.push(fs::read_to_string(&path).expect(format!("{} not found", path).as_str()));
    let path = get_config_file_path(&ConfigFile::Flatpak);
    files.push(fs::read_to_string(&path).expect(format!("{} not found", path).as_str()));
    files.join("\n")
}

fn get_cfg_diff(current: &str, lockfile: &str) {
    let diff = TextDiff::from_lines(lockfile, current);

    println!("Lines in lockfile but not in current");
    for change in diff.iter_all_changes() {
        if change.tag() == ChangeTag::Delete {
            print!("{}", change);
        }
    }
    println!("Lines in current but not in lockfile");
    for change in diff.iter_all_changes() {
        if change.tag() == ChangeTag::Insert {
            print!("{}", change);
        }
    }
}

fn get_lockfile() -> String {
    let lock_path = get_config_file_path(&ConfigFile::LockFile);
    if let Err(e) = ensure_file_exists(&lock_path) {
        eprintln!("{e}");
    }
    fs::read_to_string(&lock_path).expect("Failed to read lockfile")
}

fn generate_lockfile() {
    let lock_path = get_config_file_path(&ConfigFile::LockFile);
    if let Err(e) = ensure_file_exists(&lock_path) {
        eprintln!("{e}");
    }

    let files_string: String = get_cfg_files_string();
    if let Err(e) = fs::write(&lock_path, files_string) {
        eprintln!("{e}");
    }
}

pub fn update() {
    todo!();
}

// TODO:
// Generate lockfile for app to make sure it knows what to install or uninstall during rebuild.
pub fn rebuild() {
    let current_cfg = get_cfg_files_string();

    // check if lockfile exists yet
    let lock_path = get_config_file_path(&ConfigFile::LockFile);
    let mut lock_exists: bool = false;
    match fs::exists(lock_path) {
        Ok(true) => lock_exists = true,
        Ok(false) => lock_exists = false,
        Err(e) => eprintln!("Failed to check file: {e}"),
    }

    if lock_exists {
        get_cfg_diff(&current_cfg, &get_lockfile());
    }
}

pub fn init() {
    todo!();
}

pub fn add_flatpak_override(target: &str, flag: &str) {
    todo!();
}

pub fn rm_flatpak_override(target: &str, flag: &str) {
    todo!();
}

pub fn add_flatpak_remote(name: &str, url: &str) {
    todo!();
}

pub fn rm_flatpak_remote(name: &str, url: &str) {
    todo!();
}

pub fn enable_systemd_service(owner: &str, service: &str) {
    todo!();
}

pub fn disable_systemd_service(owner: &str, service: &str) {
    todo!();
}

pub fn mask_systemd_service(owner: &str, service: &str) {
    todo!();
}

// fn credit: gemini 3 (with manual edits, the code was broken at first :P)
fn ensure_file_exists<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    use std::fs::{File, create_dir_all};
    use std::io::ErrorKind;

    let path = path.as_ref();

    // 1. Create all parent directories if they don't exist
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    // 2. Create the file only if it is missing
    match File::create_new(path) {
        Ok(_file) => Ok(()), // Created successfully; dropped immediately
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()), // Already exists; ignore
        Err(e) => Err(e),    // Bubble up any other I/O errors
    }
}

pub fn add_packages(
    new_packages: &Vec<String>,
    config_file_type: ConfigFile,
    remote: Option<&str>,
) {
    let file_path: String = get_config_file_path(&config_file_type);

    println!("Adding packages:");
    for pkg in new_packages {
        println!("{}", pkg);
    }
    let mut doc = get_doc(&file_path);

    match config_file_type {
        Pacman => insert::pacman(&mut doc, new_packages),
        Aur => insert::aur(&mut doc, new_packages),
        Flatpak => insert::flatpak_pkg(&mut doc, new_packages, remote.unwrap()),
        _ => todo!(),
    }

    // Write changes to disk
    fs::write(file_path, doc.to_string()).expect("Failed to write file");

    // Actually run the command to install the packages
    shell::install_packages(new_packages, &config_file_type);
}

// this fn currently does not remove groups, nor can it remove a different flatpak remote package
pub fn rm_packages(new_packages: &Vec<String>, config_file_type: ConfigFile) {
    let file_path: String = get_config_file_path(&config_file_type);
    let config_file_type_str = config_file_type.as_str();

    let mut doc = get_doc(&file_path);

    let mut count: u32 = 0;
    if let Some(packages) = doc[config_file_type_str]["packages"].as_array_mut() {
        for pkg in new_packages {
            let target_index = packages.iter().position(|val| val.as_str() == Some(pkg));

            if let Some(idx) = target_index {
                count += 1;
                packages.remove(idx);
            }
        }
    }

    if count == 0 {
        println!("Package(s) not installed; nothing to do.")
    } else {
        println!("Removing packages:");
        for pkg in new_packages {
            println!("{}", pkg);
        }
    }

    // Write changes to disk
    fs::write(file_path, doc.to_string()).expect("Failed to write file");

    // Actually run the command to remove the packages
    shell::rm_packages(new_packages, &config_file_type);
}
