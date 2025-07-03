use std::fs;
use std::path::Path;
use crate::scene::data::SceneData;

pub fn save_scene(data: &SceneData) -> Result<(), std::io::Error> {
    let path = format!("scenes/{}.json", data.name);
    let json = serde_json::to_string_pretty(&data)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_scene(name: &str) -> Option<SceneData> {
    let path = format!("scenes/{}.json", name);
    if Path::new(&path).exists() {
        let json = fs::read_to_string(path).ok()?;
        serde_json::from_str(&json).ok()
    } else {
        None
    }
}

pub fn list_scenes() -> Vec<String> {
    let mut names = vec![];
    if let Ok(entries) = fs::read_dir("scenes") {
        for entry in entries.flatten() {
            if let Some(fname) = entry.path().file_stem() {
                if let Some(name) = fname.to_str() {
                    names.push(name.to_string());
                }
            }
        }
    }
    names
}