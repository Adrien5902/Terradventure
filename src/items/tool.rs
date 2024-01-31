use super::item::Item;

pub trait Tool: Item {
    fn tool_type(&self) -> ToolType;
}

pub enum ToolType {
    Sword,
    Axe,
    Pickaxe,
}
