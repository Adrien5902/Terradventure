use bevy::prelude::*;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::{collections::HashMap, fs, path::Path};

fn flatten_json(json: &Value, prefix: &str, flattened: &mut HashMap<String, String>) {
    match json {
        Value::Object(obj) => {
            for (key, value) in obj.iter() {
                let new_prefix = if prefix.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", prefix, key)
                };
                flatten_json(value, &new_prefix, flattened);
            }
        }
        _ => {
            flattened.insert(prefix.to_string(), json.to_string());
        }
    }
}

#[derive(Resource)]
pub struct Lang {
    ident: &'static str,
    name: &'static str,
    data: Option<HashMap<String, String>>,
}

impl Lang {
    fn new(ident: &'static str, name: &'static str) -> Self {
        Self {
            ident,
            name,
            data: None,
        }
    }

    fn load(mut self) -> Self {
        let data = fs::read(Path::new("assets/lang").join(format!("{}.json", self.ident))).unwrap();
        let json_str = std::str::from_utf8(&data).unwrap();
        let parsed_json: Value = serde_json::from_str(json_str).unwrap();
        let mut map = HashMap::new();
        flatten_json(&parsed_json, "", &mut map);
        self.data = Some(map);
        self
    }

    pub fn get(&self, key: &'static str) -> String {
        if let Some(d) = &self.data {
            d.get(key).unwrap().clone()
        } else {
            panic!();
        }
    }
}

pub const LANGS: Lazy<Vec<Lang>> =
    Lazy::new(|| vec![Lang::new("fr", "Français"), Lang::new("en", "English")]);
