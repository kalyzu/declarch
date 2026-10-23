use serde::Deserialize;
use std::fs;
use toml_edit::{DocumentMut, Value};

#[derive(Deserialize, Debug)]
struct Config {
    pkgs: Vec<String>,
}

fn get_doc(path: &str) -> DocumentMut {
    let toml_data = fs::read_to_string(path).expect(format!("{} not found", path).as_str());
    toml_data
        .parse::<DocumentMut>()
        .expect(format!("{} not found", path).as_str())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let conf_path = "test-data/config.toml";
    let toml_data = fs::read_to_string(conf_path)?;
    let config: Config = toml::from_str(&toml_data)?;

    println!("{:?}", config.pkgs);
    Ok(())
}

pub fn add_pkg(new_pkgs: &Vec<String>) {
    let file_path = "test-data/config.toml";

    println!("Adding packages:");
    for pkg in new_pkgs {
        println!("{}", pkg);
    }
    let mut doc = get_doc(file_path);

    if let Some(pkgs) = doc["pkgs"].as_array_mut() {
        for pkg in new_pkgs {
            let mut new_item = Value::from(pkg);
            new_item.decor_mut().set_prefix("\n    ");
            pkgs.set_trailing_comma(true);
            pkgs.push(new_item);
        }
    }

    // Write changes to disk
    fs::write(file_path, doc.to_string()).expect("Failed to write file");
}

pub fn rm_pkg(new_pkgs: &Vec<String>) {
    let file_path = "test-data/config.toml";

    println!("Removing packages:");
    for pkg in new_pkgs {
        println!("{}", pkg);
    }

    let mut doc = get_doc(file_path);

    if let Some(pkgs) = doc["pkgs"].as_array_mut() {
        for pkg in new_pkgs {
            let target_index = pkgs.iter().position(|val| val.as_str() == Some(pkg));

            if let Some(idx) = target_index {
                pkgs.remove(idx);
            }
        }
    }

    // Write changes to disk
    fs::write(file_path, doc.to_string()).expect("Failed to write file");
}
