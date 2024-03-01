use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug, fs, path::Path};
use strum_macros::{Display, EnumIter};

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
        Value::String(s) => {
            flattened.insert(prefix.to_string(), s.clone());
        }
        _ => panic!(),
    }
}

#[derive(Resource)]
pub struct Lang {
    pub ident: LangIdentifier,
    pub name: String,
    data: Option<HashMap<String, String>>,
}

impl Default for Lang {
    fn default() -> Self {
        Langs::default().into()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LangIdentifier(pub String);

#[derive(Debug, Display, Default, EnumIter)]
pub enum Langs {
    #[default]
    Français,
    English,
}

impl From<Langs> for LangIdentifier {
    fn from(value: Langs) -> Self {
        match value {
            Langs::Français => "fr",
            Langs::English => "en",
        }
        .into()
    }
}

impl From<LangIdentifier> for Langs {
    fn from(value: LangIdentifier) -> Self {
        match value.0.as_str() {
            "fr" => Self::Français,
            "en" => Self::English,
            _ => panic!("Lang not found"),
        }
    }
}

impl From<&str> for LangIdentifier {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<Lang> for Langs {
    fn from(value: Lang) -> Self {
        value.ident.into()
    }
}

impl From<Langs> for Lang {
    fn from(value: Langs) -> Self {
        let name = value.to_string();
        Self::new(value.into(), name)
    }
}

impl From<LangIdentifier> for Lang {
    fn from(value: LangIdentifier) -> Self {
        let lang: Langs = value.into();
        lang.into()
    }
}

impl Lang {
    pub fn new(ident: LangIdentifier, name: String) -> Self {
        Self {
            ident,
            name,
            data: None,
        }
    }

    pub fn load(mut self) -> Self {
        let data =
            fs::read(Path::new("assets/lang").join(format!("{}.json", self.ident.0))).unwrap();
        let json_str = std::str::from_utf8(&data).unwrap();
        let parsed_json: Value = serde_json::from_str(json_str).unwrap();
        let mut map = HashMap::new();
        flatten_json(&parsed_json, "", &mut map);
        self.data = Some(map);
        self
    }

    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        if let Some(d) = &self.data {
            d.get(key).map(|v| v.as_str()).unwrap_or(key)
        } else {
            panic!("Lang not loaded");
        }
    }
}
