/// An ascii string that holds the value "unknown"
#[allow(unsafe_code)]
pub const UNKNOWN_ID: tinystr::TinyStr16 =
    unsafe { tinystr::TinyStr16::new_unchecked(31093567915781749) };

/// A trait that helps identify which value of a type is which.
pub trait Identifiable {
    /// The type that identifies this type.
    type Id;

    const UNKNOWN: Self::Id;

    /// Get the identifier of this value.
    fn id(&self) -> &Self::Id;

    fn name(&self) -> &str;
}


//     use std::iter::FromIterator;

//     use crate::Identifiable;

//     impl<I: Identifiable, C: FromIterator<(I::Id, I)>> FromIterator<I> for C where I::Id: Clone {
//         fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
//             iter.into_iter().map(|i| (i.id().clone(), i)).collect()
//         }
//     }