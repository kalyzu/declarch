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

pub enum PkgMan {
    Pacman,
    Aur,
    Flatpak,
}

impl PkgMan {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Pacman => "pacman",
            Self::Aur => "aur",
            Self::Flatpak => "flatpak",
        }
    }
}

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

pub fn update() {
    todo!();
}

pub fn rebuild() {
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

fn get_pkg_man_path(pkg_man: &PkgMan) -> String {
    let pkg_man_str = pkg_man.as_str();
    format!("test-data/{}.toml", pkg_man_str)
}

pub fn add_packages(new_packages: &Vec<String>, pkg_man: PkgMan) {
    let file_path: String = get_pkg_man_path(&pkg_man);
    let pkg_man_str = pkg_man.as_str();

    println!("Adding packages:");
    for pkg in new_packages {
        println!("{}", pkg);
    }
    let mut doc = get_doc(&file_path);

    if let Some(packages) = doc[pkg_man_str]["packages"].as_array_mut() {
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

pub fn rm_packages(new_packages: &Vec<String>, pkg_man: PkgMan) {
    let file_path: String = get_pkg_man_path(&pkg_man);
    let pkg_man_str = pkg_man.as_str();

    let mut doc = get_doc(&file_path);

    let mut count: u32 = 0;
    if let Some(packages) = doc[pkg_man_str]["packages"].as_array_mut() {
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
}
