use std::time::Instant;

fn main() {
    println!("Building dex...");
    let start = Instant::now();
    firecore_pokedex_builder::compile("assets/pokedex/pokemon", "assets/pokedex/moves", "assets/pokedex/items");
    println!("Finished in {}ms!", start.elapsed().as_millis());
}