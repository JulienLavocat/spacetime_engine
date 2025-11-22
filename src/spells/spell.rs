use spacetimedb::{SpacetimeType, table};

use crate::math::Vec3;

pub type SpellId = u32;
pub type SchoolId = u16;
pub type TagId = u16;
pub type StatId = u16; // e.g. Hp, Mana, Strength, etc
pub type ResourceId = u16;
pub type StatusId = u16; // buff/debuff IDs
pub type EntityId = u64; // your game’s entity key

#[table(name = steng_spells)]
pub struct SpellDef {
    pub id: SpellId,
    pub name: String,
    pub school: SchoolId,  // e.g. Fire, Arcane, Support
    pub tags: Vec<TagId>,  // AoE, SingleTarget, Projectile, Dispellable, etc.
    pub cast_time_ms: i32, // 0 = instant
    pub cooldown_ms: i32,
    pub gcd_ms: i32,                       // global cooldown, if you want that
    pub range: f32,                        // generic “max range”
    pub resource_costs: Vec<ResourceCost>, // mana/energy/stamina etc
    pub targeting: TargetingSpec,
    pub effects: Vec<EffectDef>,
}

#[derive(SpacetimeType)]
pub struct ResourceCost {
    pub resource: ResourceId,
    pub amount: i32,
}

#[derive(SpacetimeType)]
pub enum TargetingSpec {
    SelfOnly,
    SingleAlly,
    SingleEnemy,
    Position(PositionTargeting),
    Area(AreaTargeting),
}

#[derive(SpacetimeType)]
pub struct PositionTargeting {
    pub max_radius: f32,
}

#[derive(SpacetimeType)]
pub struct AreaTargeting {
    pub shape: AreaShape,
    pub radius: f32,
    pub max_range: f32,
}

#[derive(SpacetimeType)]
pub enum AreaShape {
    Circle(f32),
    Rectangle(Vec3), // width, height no Z -> Should I get a Vec2 ?
}

#[derive(SpacetimeType)]
pub struct EffectDef {
    pub id: u8,             // index within spell
    pub when: EffectTiming, // OnCast, OnHit, Periodic tick, OnExpire…
    pub kind: EffectKind,
}

#[derive(SpacetimeType)]
pub enum EffectTiming {
    OnCast,
    OnCastComplete,
    OnHit,
    Periodic(PeriodicEffectTiming),
    OnExpire,
}

#[derive(SpacetimeType)]
pub struct PeriodicEffectTiming {
    pub interval_ms: i32,
    pub duration_ms: i32,
}

#[derive(SpacetimeType)]
pub enum EffectKind {
    Damage(DamageEffect),
    Heal(HealEffect),
    ModifyStat(ModifyStatEffect),
    ApplyStatus(ApplyStatusEffect),
    RemoveStatus(RemoveStatusEffect),
    Dispel(DispelEffect),
    TeleportToTarget, // interpret with game hooks
    Knockback(KnockbackEffect),
    SummonEntity(SummonEntityEffect),
    Custom(CustomEffect), // extension point for game-specific logic
}

#[derive(SpacetimeType)]
pub struct DamageEffect {
    pub amount: i32,
    pub damage_type: SchoolId,
}

#[derive(SpacetimeType)]
pub struct HealEffect {
    pub amount: AmountFormula,
}

#[derive(SpacetimeType)]
pub struct ModifyStatEffect {
    pub stat: StatId,
    pub delta: AmountFormula,
}

#[derive(SpacetimeType)]
pub struct ApplyStatusEffect {
    pub status_id: StatusId,
    pub duration_ms: i32,
    pub stacks: i32,
}

#[derive(SpacetimeType)]
pub struct RemoveStatusEffect {
    pub status_id: StatusId,
    pub stacks: i32,
}

#[derive(SpacetimeType)]
pub struct DispelEffect {
    pub max_statuses: u8,
    pub filter_tags: Vec<TagId>,
}

#[derive(SpacetimeType)]
pub struct KnockbackEffect {
    pub distance: f32,
}

#[derive(SpacetimeType)]
pub struct SummonEntityEffect {
    pub archetype_id: u32,
    pub count: u8,
}

#[derive(SpacetimeType)]
pub struct CustomEffect {
    pub code: u16,
}

#[derive(SpacetimeType)]
pub enum AmountFormula {
    Flat(i32),
    StatScale(StatScaled),
    AttackerVsDefender(AttackerVsDefender),
    // You can add “RandomRange” or “CritChance” as needed via the engine.
}

#[derive(SpacetimeType)]
pub struct StatScaled {
    pub stat: StatId,
    pub coeff: f32,
    pub base: i32,
}

#[derive(SpacetimeType)]
pub struct AttackerVsDefender {
    pub attacker_stat: StatId,
    pub defender_stat: StatId,
    pub coeff: f32,
    pub base: i32,
}

#[table(name = steng_spells_active_casts)]
pub struct ActiveCast {
    pub id: u64,
    pub caster: EntityId,
    pub spell_id: SpellId,
    pub target: TargetInstance,
    pub started_at_ms: i64,
    pub will_end_at_ms: i64,
    pub flags: CastFlags,
}

#[derive(SpacetimeType)]
pub struct CastFlags {
    pub is_channel: bool,
    pub interrupted: bool,
}

#[derive(SpacetimeType)]
pub enum TargetInstance {
    OnSelf,
    Entity(EntityId),
    Position(Vec3),
}

#[table(name = steng_spells_cooldowns)]
pub struct Cooldown {
    pub caster: EntityId,
    pub spell_id: SpellId,
    pub ready_at_ms: i64,
}

#[table(name = steng_spells_status_instances)]
pub struct StatusInstance {
    pub id: u64,
    pub status_id: StatusId,
    pub owner: EntityId,
    pub source: EntityId, // caster
    pub applied_at_ms: i64,
    pub expires_at_ms: i64,
    pub stacks: i32,
    pub periodic_interval_ms: Option<i32>,
    pub next_tick_at_ms: Option<i64>,
    pub spell_id: Option<SpellId>, // for reference
}
