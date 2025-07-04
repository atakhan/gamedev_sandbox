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
    let path = std::path::Path::new("scenes");
    if !path.exists() {
        return vec![];
    }

    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter_map(|e| e.path().file_stem()?.to_str().map(|s| s.to_string()))
            .collect(),
        Err(_) => vec![],
    }
}