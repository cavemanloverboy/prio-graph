use crate::{priority_id::PriorityId, resource_key::ResourceKey};

pub enum AccessKind {
    Read,
    Write,
}

pub trait Transaction<Id: PriorityId, Rk: ResourceKey> {
    fn id(&self) -> Id;
    fn check_resource_keys<F: FnMut(&Rk, AccessKind)>(&self, checker: F);
}
