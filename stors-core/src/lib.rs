mod item_impls;

pub use item_impls::item;

pub trait Id {
    fn id(&self) -> uuid::Uuid;
}
