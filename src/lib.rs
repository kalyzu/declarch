// use serde::Deserialize;
use std::fs;
use toml_edit::{DocumentMut, Value};

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

fn get_doc(path: &str) -> DocumentMut {
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

pub fn add_pkg(new_packages: &Vec<String>) {
    let file_path = "test-data/config.toml";

    println!("Adding packages:");
    for pkg in new_packages {
        println!("{}", pkg);
    }
    let mut doc = get_doc(file_path);

    if let Some(packages) = doc["pacman"]["packages"].as_array_mut() {
        for pkg in new_packages {
            let mut new_item = Value::from(pkg);
            new_item.decor_mut().set_prefix("\n    ");
            packages.set_trailing_comma(true);
            packages.push(new_item);
        }
    }

    // Write changes to disk
    fs::write(file_path, doc.to_string()).expect("Failed to write file");
}

pub fn rm_pkg(new_packages: &Vec<String>) {
    let file_path = "test-data/config.toml";

    println!("Removing packages:");
    for pkg in new_packages {
        println!("{}", pkg);
    }

    let mut doc = get_doc(file_path);

    if let Some(packages) = doc["pacman"]["packages"].as_array_mut() {
        for pkg in new_packages {
            let target_index = packages.iter().position(|val| val.as_str() == Some(pkg));

            if let Some(idx) = target_index {
                packages.remove(idx);
            }
        }
    }

    // Write changes to disk
    fs::write(file_path, doc.to_string()).expect("Failed to write file");
}
