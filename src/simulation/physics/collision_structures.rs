use std::hash::Hash;

use dashmap::DashSet;
use fxhash::FxBuildHasher;
use legion::*;

pub type CollisionSet = DashSet<Collision, FxBuildHasher>;

#[derive(Eq)]
pub struct Collision {
    pub a: Entity,
    pub b: Entity,
}

impl Collision {
    pub fn new(a: Entity, b: Entity) -> Self {
        Self { a, b }
    }
}

impl PartialEq for Collision {
    fn eq(&self, other: &Self) -> bool {
        ((self.a == other.a) & (self.b == other.b)) || ((self.a == other.b) & (self.b == other.a))
    }
}

impl Hash for Collision {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
    }
}
