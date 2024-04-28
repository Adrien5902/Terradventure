use super::item::ItemTrait;

pub trait Tool: ItemTrait {
    fn tool_type(&self) -> ToolType;
}

pub enum ToolType {
    Sword,
    Axe,
    Pickaxe,
}
