use std::time::Instant;

use dex_builder::error::EntryError;

fn main() -> Result<(), EntryError> {
    println!("Building dex...");
    let start = Instant::now();
    let bytes = dex_builder::compile("pokedex/pokemon", "pokedex/moves", "output/dex.bin", false)?;
    println!("Wrote {} bytes in {}ms!", bytes, start.elapsed().as_millis());
    Ok(())
}