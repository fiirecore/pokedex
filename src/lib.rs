pub mod ailment;
pub mod item;
pub mod moves;
pub mod pokemon;
pub mod types;

mod dex;
pub use dex::*;

pub mod id;
pub use id::{IdRef, Identifiable};

fn name<T: ?Sized>() -> &'static str {
    let name = core::any::type_name::<T>();
    name.split("::").last().unwrap_or(name)
}