use self::swordsman::Swordsman;

pub mod swordsman;

pub trait PlayerClass: Sync + Send {
    fn name(&self) -> &'static str;
}

impl Default for Box<dyn PlayerClass> {
    fn default() -> Self {
        Box::new(Swordsman)
    }
}
