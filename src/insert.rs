use toml_edit::{Array, DocumentMut, Value, table, value};

use crate::ConfigFile;
use crate::shell;

pub fn pacman(doc: &mut DocumentMut, packages: &Vec<String>) {
    // Verify that the pacman table exists
    doc.entry("pacman").or_insert(table());
    // Verify that packages under pacman exists
    {
        let pacman = doc["pacman"].as_table_mut().unwrap();
        pacman
            .entry("packages")
            .or_insert_with(|| value(Array::new()));
    }

    // This whole if let block is just for groups.
    if let Some(groups) = shell::check_for_groups(&packages, ConfigFile::Pacman) {
        for (group_name, group_pkgs) in groups {
            let group_table_name: String = format!("pacman.groups.{}", group_name);
            // Verify that the group table exists
            doc.entry(&group_table_name).or_insert(table());
            // Verify that packages under group table exists
            {
                let group_table = doc[&group_table_name].as_table_mut().unwrap();
                group_table
                    .entry("packages")
                    .or_insert_with(|| value(Array::new()));
            }
            // add to toml any groups
            if let Some(packages_array) = doc[&group_table_name]["packages"].as_array_mut() {
                for pkg in group_pkgs {
                    if !packages_array.iter().any(|v| v.as_str() == Some(&pkg)) {
                        let mut new_item = Value::from(pkg);
                        new_item.decor_mut().set_prefix("\n    ");
                        packages_array.set_trailing_comma(true);
                        packages_array.push(new_item);
                    }
                }
            }
        }
    }
    // Now that groups are out of the way, we can deal with regular pacman packages
    if let Some(packages_array) = doc["pacman"]["packages"].as_array_mut() {
        for pkg in packages {
            // Prevent duplicate package additions if it's already there
            if !packages_array.iter().any(|v| v.as_str() == Some(&pkg)) {
                let mut new_item = Value::from(pkg.clone());
                new_item.decor_mut().set_prefix("\n    ");
                packages_array.set_trailing_comma(true);
                packages_array.push(new_item);
            }
        }
    }
}

pub fn aur_command(packages_string: &str, config_file_type: &ConfigFile) -> String {
    let file_path = crate::get_config_file_path(&config_file_type);
    let mut doc = crate::get_doc(&file_path);
    // Verify that the aur table exists
    doc.entry("aur").or_insert(table());
    // Verify that helper under aur exists
    let helper = doc["aur"]["helper"].as_str().unwrap_or("paru");
    format!("{helper} -Si {packages_string}")
}

pub fn aur(doc: &mut DocumentMut, packages: &Vec<String>) {}

pub fn flatpak_pkg(doc: &mut DocumentMut, packages: &Vec<String>) {}
