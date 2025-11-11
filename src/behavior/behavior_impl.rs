use ai_behavior::{Behavior, Status};
use serde::de::DeserializeOwned;
use spacetimedb::ReducerContext;

use crate::{
    behavior::behavior_state::{BehaviorTree, BehaviorTreeId},
    utils::Entity,
    world::World,
};

pub trait BehaviorExecutor<T> {
    fn run_action(
        &mut self,
        ctx: &ReducerContext,
        world: &World,
        delta_time: f32,
        action: &T,
    ) -> Status;
}

pub fn tick_behavior<'a, T, E>(
    ctx: &ReducerContext,
    world: &World,
    tree_id: BehaviorTreeId,
    delta_time: f32,
    states: &mut [E],
) where
    T: DeserializeOwned,
    E: BehaviorExecutor<T> + 'a,
{
    let bt = BehaviorTree::find(ctx, tree_id).expect("BehaviorTree not found");
    let (behavior, _): (Behavior<T>, _) =
        bincode::serde::decode_from_slice(&bt.behavior, bincode::config::standard())
            .expect("Failed to deserialize behavior tree");

    for entity_exec in states.iter_mut() {
        execute_behavior_tree(ctx, world, delta_time, &behavior, entity_exec);
    }
}

fn execute_behavior_tree<T>(
    ctx: &ReducerContext,
    world: &World,
    delta_time: f32,
    behavior: &Behavior<T>,
    exec: &mut impl BehaviorExecutor<T>,
) -> Status {
    match behavior {
        Behavior::Action(action) => exec.run_action(ctx, world, delta_time, action),
        Behavior::Sequence(children) => {
            for child in children {
                match execute_behavior_tree(ctx, world, delta_time, child, exec) {
                    Status::Success => continue,
                    other => return other,
                }
            }
            Status::Success
        }
        Behavior::Select(children) => {
            for child in children {
                match execute_behavior_tree(ctx, world, delta_time, child, exec) {
                    Status::Failure => continue,
                    other => return other,
                }
            }
            Status::Failure
        }
        Behavior::If(cond, success, failure) => {
            if execute_behavior_tree(ctx, world, delta_time, cond, exec) == Status::Success {
                execute_behavior_tree(ctx, world, delta_time, success, exec)
            } else {
                execute_behavior_tree(ctx, world, delta_time, failure, exec)
            }
        }

        Behavior::Fail(child) => match execute_behavior_tree(ctx, world, delta_time, child, exec) {
            Status::Success => Status::Failure,
            Status::Failure => Status::Success,
            Status::Running => Status::Running,
        },
        Behavior::AlwaysSucceed(child) => {
            match execute_behavior_tree(ctx, world, delta_time, child, exec) {
                Status::Success => Status::Success,
                Status::Failure => Status::Success,
                Status::Running => Status::Running,
            }
        }
        _ => {
            panic!(
                "Unsupported behavior node type, only Action, Sequence, If, and Select are supported"
            );
        }
    }
}
