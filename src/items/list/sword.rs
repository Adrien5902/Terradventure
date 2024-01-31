use crate::items::{
    item::{Item, ItemName, ItemTexture, StackSize},
    tool::{Tool, ToolType},
};

pub struct Sword;

impl Item for Sword {
    fn name(&self) -> ItemName {
        ItemName::from("sword")
    }

    fn texture(&self) -> ItemTexture {
        ItemTexture::from("sword")
    }

    fn stack_size(&self) -> StackSize {
        1
    }
}

impl Tool for Sword {
    fn tool_type(&self) -> crate::items::tool::ToolType {
        ToolType::Sword
    }
}
