pub type StackSize = u8;
const MAX_STACK_SIZE: StackSize = StackSize::MAX;

pub trait Item: Sync {
    fn texture(&self) -> ItemTexture;
    fn name(&self) -> ItemName;
    fn stack_size(&self) -> StackSize {
        MAX_STACK_SIZE
    }
    // fn get_use(&self) -> Option<fn() -> ()> {
    //     None
    // }
}

pub struct ItemTexture(String);

impl From<&str> for ItemTexture {
    fn from(value: &str) -> Self {
        Self(format!("textures/items/{}.png", value))
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ItemName(String);

impl From<&str> for ItemName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl ItemName {
    pub fn get(&self) -> String {
        self.0.clone()
    }
}
