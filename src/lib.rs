pub mod item;
pub mod moves;
pub mod pokemon;
pub mod ailment;
pub mod types;

mod id;
pub use id::*;

fn name<T: ?Sized>() -> &'static str {
    let name = core::any::type_name::<T>();
    name.split("::").last().unwrap_or(name)
}