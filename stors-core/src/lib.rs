//! # Stors
//!
//! An online store as a pet project.
//!
//! It implements
//! - Hexagonal Architecture
//! - Event sourcing
//! - CQRS

mod item_impls;

pub use item_impls::item;

pub trait Id {
    fn id(&self) -> uuid::Uuid;
}
