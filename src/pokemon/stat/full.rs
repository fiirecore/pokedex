use serde::{Deserialize, Serialize};
use super::{StatType, StatSet};
use core::{fmt::{Debug, Display, Formatter, Result as FmtResult}, ops::{Deref, DerefMut}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FullStatType {
    Basic(StatType),
    Accuracy,
    Evasion,
}

impl Display for FullStatType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FullStatType::Basic(basic) => Debug::fmt(basic, f),
            other => Debug::fmt(other, f),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct FullStatSet<S> {
    pub basic: StatSet<S>,
    pub accuracy: S,
    pub evasion: S,
}

impl<S: Sized + Copy> FullStatSet<S> {
    pub fn uniform(stat: S) -> Self {
        Self {
            basic: StatSet::uniform(stat),
            accuracy: stat,
            evasion: stat,
        }
    }

    pub fn get(&self, stat: FullStatType) -> &S {
        match stat {
            FullStatType::Basic(stat) => self.basic.get(stat),
            FullStatType::Accuracy => &self.accuracy,
            FullStatType::Evasion => &self.evasion,
        }
    }

    pub fn get_mut(&mut self, stat: FullStatType) -> &mut S {
		match stat {
			FullStatType::Basic(stat) => self.basic.get_mut(stat),
			FullStatType::Accuracy => &mut self.accuracy,
			FullStatType::Evasion => &mut self.evasion,
		}
	}
}

impl<S> Deref for FullStatSet<S> {
    type Target = StatSet<S>;

    fn deref(&self) -> &Self::Target {
        &self.basic
    }
}

impl<S> DerefMut for FullStatSet<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.basic
    }
}