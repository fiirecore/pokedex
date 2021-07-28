pub mod battle;
pub mod id;
pub mod item;
pub mod moves;
pub mod pokemon;
pub mod status;
pub mod trainer;
pub mod types;

pub const UNKNOWN_ID: tinystr::TinyStr16 = unsafe { tinystr::TinyStr16::new_unchecked(31093567915781749) };

/*

pub mod borrow {

    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::{
        borrow::Cow,
        ops::{Deref, DerefMut},
    };

    #[derive(Debug)]
    pub enum BorrowableMut<'a, T> {
        Owned(T),
        Borrowed(&'a mut T),
    }

    impl<'a, T: Clone> BorrowableMut<'a, T> {

        pub fn owned(self) -> T {
            match self {
                BorrowableMut::Owned(t) => t,
                BorrowableMut::Borrowed(t) => t.clone(),
            }
        }

    }

    impl<'a, T: ToOwned> BorrowableMut<'a, T> {
        pub fn as_ref(&'a self) -> Cow<'a, T> {
            match self {
                Self::Borrowed(t) => Cow::Borrowed(t),
                Self::Owned(t) => Cow::Owned(t.to_owned()),
            }
        }
    }

    impl<'a, T> Deref for BorrowableMut<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            match self {
                Self::Owned(instance) => instance,
                Self::Borrowed(instance) => &**instance,
            }
        }
    }

    impl<'a, T> DerefMut for BorrowableMut<'a, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            match self {
                Self::Owned(instance) => instance,
                Self::Borrowed(instance) => *instance,
            }
        }
    }

    impl<'a, T: Clone> Clone for BorrowableMut<'a, T> {
        fn clone(&self) -> Self {
            Self::Owned(self.deref().clone())
        }
    }

    impl<'de, 'a, T: Deserialize<'de>> Deserialize<'de> for BorrowableMut<'a, T> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            T::deserialize(deserializer).map(BorrowableMut::Owned)
        }
    }

    impl<'a, T: Serialize> Serialize for BorrowableMut<'a, T> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.deref().serialize(serializer)
        }
    }
}

*/