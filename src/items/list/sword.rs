use serde::{Deserialize, Serialize};

use crate::items::{
    item::{Item, ItemName, StackSize},
    tool::{Tool, ToolType},
};

#[derive(Clone, Deserialize, Serialize)]
pub struct Sword;

impl Item for Sword {
    fn name(&self) -> ItemName {
        "sword".into()
    }

    fn stack_size(&self) -> StackSize {
        StackSize::MIN
    }
}

impl Tool for Sword {
    fn tool_type(&self) -> crate::items::tool::ToolType {
        ToolType::Sword
    }
}
