use serde::{Deserialize, Serialize};
use super::{MoveId, PP};

pub type SavedMoveSet = util::smallvec::SmallVec<[SavedMove; 4]>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SavedMove {
	pub id: MoveId,
	pub pp: Option<PP>,
}