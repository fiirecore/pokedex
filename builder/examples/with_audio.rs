use std::time::Instant;

use dex_builder::error::EntryError;

fn main() -> Result<(), EntryError> {
    println!("Building dex...");
    let start = Instant::now();
    dex_builder::compile("pokedex/pokemon", "pokedex/moves", "output/dex.bin", true);
    println!("Finished in {}ms!", start.elapsed().as_millis());
    Ok(())
}