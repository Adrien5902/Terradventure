use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug, fs, path::Path};
use strum::IntoEnumIterator;
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
        LangIdentifier::default().into()
    }
}

#[derive(Debug, Display, Default, EnumIter, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LangIdentifier {
    #[default]
    Français,
    English,
}

impl Into<String> for LangIdentifier {
    fn into(self: LangIdentifier) -> String {
        match self {
            LangIdentifier::Français => "fr",
            LangIdentifier::English => "en",
        }
        .to_owned()
    }
}

impl From<String> for LangIdentifier {
    fn from(value: String) -> Self {
        let pairs: Vec<(LangIdentifier, String)> = LangIdentifier::iter()
            .map(|ident| (ident, ident.into()))
            .collect();

        pairs
            .into_iter()
            .find_map(|(ident, str)| (*str == value).then_some(ident))
            .unwrap()
    }
}

impl From<Lang> for LangIdentifier {
    fn from(value: Lang) -> Self {
        value.ident.into()
    }
}

impl From<LangIdentifier> for Lang {
    fn from(value: LangIdentifier) -> Self {
        let name: String = value.to_string();
        Self::new(value, name)
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
        let ident: String = self.ident.into();
        let data =
            fs::read_to_string(Path::new("assets/lang").join(format!("{ident}.json",))).unwrap();
        let parsed_json: Value = serde_json::from_str(&data).unwrap();
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
