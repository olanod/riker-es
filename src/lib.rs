#[macro_use]
extern crate log;

pub use entity::{Aggregate, Entity, Query, StoreRef, Sys, CQRS, ES};
use riker::actors::ChannelRef;
pub use riker_patterns::ask::ask;
use std::fmt;
pub use store::{Commit, Store, StoreMsg};
use uuid::Uuid;

pub type EventBus<T> = ChannelRef<Event<T>>;

mod entity;
mod store;

/// Events are changes to the system generated by entities after processing
/// other events or external commands
#[derive(Clone, Debug)]
pub enum Event<T: Aggregate> {
    Create(T),
    Update(EntityId, T::Update),
}
impl<T: Aggregate> Event<T> {
    pub fn entity_id(&self) -> EntityId {
        match self {
            Event::Create(e) => e.id(),
            Event::Update(id, _) => *id,
        }
    }
}
impl<T: Aggregate> From<(EntityId, T::Update)> for Event<T> {
    fn from((id, data): (EntityId, T::Update)) -> Self {
        Event::Update(id, data)
    }
}
impl<T: Aggregate> From<T> for Event<T> {
    fn from(data: T) -> Self {
        Event::Create(data)
    }
}

/// Uniquely idenfies an entity
#[derive(Clone, Debug, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EntityId(Uuid);
impl EntityId {
    pub fn new() -> Self {
        Default::default()
    }
}
impl From<&str> for EntityId {
    fn from(id: &str) -> Self {
        EntityId(Uuid::new_v5(&Uuid::NAMESPACE_URL, id.as_bytes()))
    }
}
impl From<Uuid> for EntityId {
    fn from(uuid: Uuid) -> Self {
        EntityId(uuid)
    }
}
impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Default for EntityId {
    fn default() -> Self {
        Uuid::new_v4().into()
    }
}
