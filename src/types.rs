use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Mul,
};
use serde::{Deserialize, Serialize};

/// Pokemon types
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum PokemonType {
    Unknown,

    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

/// Pokemon Type effectiveness
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Effective {
    Effective,
    Ineffective,
    NotEffective,
    SuperEffective,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct PokemonTypes {
    pub primary: PokemonType,
    #[serde(default)]
    pub secondary: Option<PokemonType>,
}

impl Effective {
    /// The multiplier of an effect a pokemon type would have on another pokemon type
    pub const fn multiplier(self) -> f32 {
        match self {
            Effective::Ineffective => 0.0,
            Effective::NotEffective => 0.5,
            Effective::Effective => 1.0,
            Effective::SuperEffective => 2.0,
        }
    }
}

impl Default for PokemonType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for Effective {
    fn default() -> Self {
        Self::Effective
    }
}

impl Mul for Effective {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Ineffective => Self::Ineffective,
            Self::NotEffective => match rhs {
                Self::SuperEffective => Self::Effective,
                Self::Ineffective => Self::Ineffective,
                _ => Self::NotEffective,
            },
            Self::Effective => rhs,
            Self::SuperEffective => match rhs {
                Self::NotEffective => Self::Effective,
                Self::Ineffective => Self::Ineffective,
                _ => Self::SuperEffective,
            },
        }
    }
}

impl Display for Effective {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Effective::Ineffective => "ineffective",
                Effective::NotEffective => "not very effective",
                Effective::Effective => "effective",
                Effective::SuperEffective => "super effective",
            }
        )
    }
}
