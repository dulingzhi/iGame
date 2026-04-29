//! iGame Editor — stub binary.
//!
//! Full editor implementation planned in M3 (see ROADMAP.md).
//! Currently prints package info for the given map path.

use igame_shared::map_package::MapPackage;
use igame_shared::validate;
use std::path::PathBuf;

fn main() {
    let map_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "assets/maps/demo".to_string());

    println!("iGame Editor (stub)");
    println!("==================");
    println!("Map path: {map_path}");

    match MapPackage::load(&PathBuf::from(&map_path)) {
        Ok(pkg) => {
            println!("Loaded: {} v{}", pkg.manifest.name, pkg.manifest.version);
            if let Some(author) = &pkg.manifest.author {
                println!("Author: {author}");
            }
            println!("Entities: {}", pkg.scene.entities.len());
            match validate(&pkg) {
                Ok(()) => println!("Validation: OK"),
                Err(e) => println!("Validation error: {e}"),
            }
        }
        Err(e) => {
            eprintln!("Error loading map package: {e}");
            std::process::exit(1);
        }
    }

    println!("\nFull editor UI coming in M3 — see ROADMAP.md");
}
