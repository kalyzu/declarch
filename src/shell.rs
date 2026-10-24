// All shell commands performed here only
use crate::ConfigFile;
use crate::insert::aur_command;
use std::{collections::HashMap, process::Command};

pub fn check_for_groups(
    packages: &Vec<String>,
    config_file_type: ConfigFile,
) -> Option<HashMap<String, Vec<String>>> {
    let packages_string: String = packages.join(" ");

    let group_command = match &config_file_type {
        ConfigFile::Pacman => format!("pacman -Sg {}", packages_string),
        ConfigFile::Aur => aur_command(&packages_string, &config_file_type),
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
