//! Dump USD scene hierarchy example.
//!
//! Traverses a USD file and prints the scene tree with indentation.
//!
//! Usage: cargo run --example dump_hierarchy -- <path/to/file.usdz>

use std::env;
use tinyusdz_rs::Stage;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path/to/file.usd[z]>", args[0]);
        eprintln!("Example: {} scene.usdz", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    println!("Loading: {}", path);
    println!();

    // Load the stage
    let stage = match Stage::open(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error loading USD file: {}", e);
            std::process::exit(1);
        }
    };

    println!("Scene Hierarchy:");
    println!("================");

    // Traverse all prims and print with indentation
    // Note: This simplified version doesn't track actual hierarchy depth
    // A full implementation would need to track parent-child relationships
    for prim in stage.traverse() {
        let type_name = prim.type_name();
        let name = prim.name();
        let num_children = prim.num_children();
        let num_props = prim.property_names().len();

        // Print prim info
        let type_str = if type_name.is_empty() {
            String::from("(no type)")
        } else {
            type_name.to_string()
        };

        println!(
            "/{} <{}>  [{} children, {} properties]",
            name, type_str, num_children, num_props
        );

        // Print properties
        let props = prim.property_names();
        if !props.is_empty() {
            for prop in props.iter().take(10) {
                // Limit to first 10 properties
                println!("    .{}", prop);
            }
            if props.len() > 10 {
                println!("    ... and {} more properties", props.len() - 10);
            }
        }
    }

    println!();
    println!("Done!");
}
