// All shell commands performed here only
use crate::ConfigFile;
use crate::insert::aur_helper;
use std::{collections::HashMap, process::Command};

pub fn install_packages(packages: &Vec<String>, config_file_type: &ConfigFile) {
    let packages_string: String = packages.join(" ");
    let install_command = match &config_file_type {
        ConfigFile::Pacman => {
            format!("sudo pacman -Sy && sudo pacman -S --needed {packages_string}")
        }
        ConfigFile::Aur => format!(
            "{} -S --needed {packages_string}",
            aur_helper(&config_file_type)
        ),
        ConfigFile::Flatpak => todo!(),
    };

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(install_command)
        .spawn()
        .expect("Failed to run command!");
    child.wait().expect("Failed to run command!");
}

pub fn rm_packages(packages: &Vec<String>, config_file_type: &ConfigFile) {
    let packages_string: String = packages.join(" ");
    let install_command = match &config_file_type {
        ConfigFile::Pacman => format!("sudo pacman -Rns {packages_string}"),
        ConfigFile::Aur => format!("{} -Rns {packages_string}", aur_helper(&config_file_type)),
        ConfigFile::Flatpak => todo!(),
    };

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(install_command)
        .spawn()
        .expect("Failed to run command!");
    child.wait().expect("Failed to run command!");
}

pub fn check_for_groups(
    packages: &Vec<String>,
    config_file_type: ConfigFile,
) -> Option<HashMap<String, Vec<String>>> {
    let packages_string: String = packages.join(" ");

    let group_command = match &config_file_type {
        ConfigFile::Pacman => format!("pacman -Sg {}", packages_string),
        ConfigFile::Aur => format!("{} -Si {packages_string}", aur_helper(&config_file_type)),
        _ => return None,
    };

    let output = Command::new("sh")
        .arg("-c")
        .arg(group_command)
        .output()
        .expect("Failed to run command!");

    let stdout_string = String::from_utf8_lossy(&output.stdout);

    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    match &config_file_type {
        ConfigFile::Pacman => {
            for line in stdout_string.lines() {
                let mut parts = line.split_whitespace();
                if let (Some(group), Some(pkg)) = (parts.next(), parts.next()) {
                    groups
                        .entry(group.to_string())
                        .or_default()
                        .push(pkg.to_string());
                }
            }

            if groups.len() == 0 {
                None
            } else {
                Some(groups)
            }
        }
        ConfigFile::Aur => {
            todo!()
        }
        _ => return None,
    }
}
