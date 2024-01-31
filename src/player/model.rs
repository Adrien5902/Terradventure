use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct PlayerModel(&'static str);

impl Default for PlayerModel {
    fn default() -> Self {
        PLAYER_MODELS[0].clone()
    }
}

static PLAYER_MODELS: Lazy<Vec<PlayerModel>> = Lazy::new(|| {
    ["default"]
        .iter()
        .map(|s| PlayerModel::from(*s))
        .collect::<Vec<_>>()
});

impl From<&'static str> for PlayerModel {
    fn from(value: &'static str) -> Self {
        PlayerModel(value)
    }
}
