use std::collections::HashMap;

use ai_behavior::Behavior;
use serde::{Serialize, de::DeserializeOwned};
use spacetimedb::{ReducerContext, Table, table};

use crate::utils::Entity;

pub type BehaviorTreeId = u64;

#[table(name = steng_behavior_tree)]
pub struct BehaviorTree {
    #[primary_key]
    #[auto_inc]
    pub id: BehaviorTreeId,
    pub behavior: Vec<u8>,
}

impl Entity for BehaviorTree {
    fn insert(self, ctx: &spacetimedb::ReducerContext) -> Self {
        ctx.db.steng_behavior_tree().insert(self)
    }

    fn find(ctx: &spacetimedb::ReducerContext, id: u64) -> Option<Self> {
        ctx.db.steng_behavior_tree().id().find(id)
    }

    fn iter(ctx: &spacetimedb::ReducerContext) -> impl Iterator<Item = Self> {
        ctx.db.steng_behavior_tree().iter()
    }

    fn as_map(ctx: &spacetimedb::ReducerContext) -> std::collections::HashMap<u64, Self> {
        ctx.db
            .steng_behavior_tree()
            .iter()
            .map(|entry| (entry.id, entry))
            .collect()
    }

    fn as_vec(ctx: &spacetimedb::ReducerContext) -> Vec<Self> {
        ctx.db.steng_behavior_tree().iter().collect()
    }

    fn update(self, ctx: &spacetimedb::ReducerContext) -> Self {
        ctx.db.steng_behavior_tree().id().update(self)
    }

    fn delete(&self, ctx: &spacetimedb::ReducerContext) {
        ctx.db.steng_behavior_tree().id().delete(self.id);
    }

    fn clear(ctx: &spacetimedb::ReducerContext) {
        ctx.db.steng_behavior_tree().iter().for_each(|entry| {
            ctx.db.steng_behavior_tree().id().delete(entry.id);
        });
    }

    fn count(ctx: &ReducerContext) -> usize {
        ctx.db.steng_behavior_tree().iter().count()
    }
}

impl BehaviorTree {
    pub fn create<T: Serialize>(ctx: &ReducerContext, behavior: Behavior<T>) -> Self {
        let behavior = bincode::serde::encode_to_vec(&behavior, bincode::config::standard())
            .expect("Failed to serialize behavior tree");
        Self { id: 0, behavior }.insert(ctx)
    }

    pub fn load<T: DeserializeOwned>(&self) -> Behavior<T> {
        let (behavior, _): (Behavior<T>, _) =
            bincode::serde::decode_from_slice(&self.behavior, bincode::config::standard())
                .expect("Failed to deserialize behavior tree");
        behavior
    }

    pub fn load_all<T: DeserializeOwned>(
        ctx: &ReducerContext,
    ) -> HashMap<BehaviorTreeId, Behavior<T>> {
        BehaviorTree::iter(ctx)
            .map(|entry| (entry.id, entry.load()))
            .collect()
    }
}
