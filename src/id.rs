/// An ascii string that holds the value "unknown"
#[allow(unsafe_code)]
pub const UNKNOWN_ID: tinystr::TinyStr16 =
    unsafe { tinystr::TinyAsciiStr::from_bytes_unchecked(31093567915781749u128.to_ne_bytes()) };

/// A trait that helps identify which value of a type is which.
pub trait Identifiable {
    /// The type that identifies this type.
    type Id;

    const UNKNOWN: Self::Id;

    /// Get the identifier of this value.
    fn id(&self) -> &Self::Id;
}

pub trait Nameable: Identifiable {

    fn name(&self) -> &str;

}

//     use std::iter::FromIterator;

//     use crate::Identifiable;

//     impl<I: Identifiable, C: FromIterator<(I::Id, I)>> FromIterator<I> for C where I::Id: Clone {
//         fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
//             iter.into_iter().map(|i| (i.id().clone(), i)).collect()
//         }
//     }