mod entities;
mod parsing;
mod bug;

use parsing::{decode_blueprint, get_recipes_map};

fn main() {
    let recipes = get_recipes_map().unwrap();
    //println!("{:?}", recipes);

    let encoded = include_str!("../blueprint.b64");
    match decode_blueprint(encoded) {
        Ok(decoded) => println!("Decoded: {:?}", decoded),
        Err(e) => eprintln!("Error decoding: {}", e),
    }
}
