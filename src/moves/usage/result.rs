use crate::{
    pokemon::{stat::{StatType, Stage}, Health},
    ailment::LiveAilment,
};

use super::{DamageResult, Targets};

pub type MoveResults = Targets<Vec<MoveResult>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    Damage(DamageResult<Health>),
    Ailment(LiveAilment),
    Drain(DamageResult<Health>, i16), // damage, health gained/lost
    Stat(StatType, Stage),
    Accuracy(Stage),
    Evasion(Stage),
    Flinch,
    NoHit(NoHitResult),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoHitResult {
    Ineffective,
    Miss,
    Todo,
    Error,
}
