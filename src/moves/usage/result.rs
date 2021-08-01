use crate::{
    pokemon::{stat::StatStage, Health},
    status::StatusEffectInstance,
};

use super::DamageResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    Damage(DamageResult<Health>), // bool = crit
    Status(StatusEffectInstance),
    Drain(DamageResult<Health>, i16), // damage, health gained/lost
    StatStage(StatStage),
    Flinch,
    // NextHit(), next hit protect, next hit endure
    NoHit(NoHitResult),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoHitResult {
    Ineffective,
    Miss,
    Todo,
}
