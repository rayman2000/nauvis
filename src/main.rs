mod bug;
mod entities;
mod parsing;
mod type_check;
mod leak;

use bug::check_for_bugs;
use parsing::{decode_blueprint, get_recipes_map};

fn main() {
    let recipes = get_recipes_map().unwrap();
    //println!("{:?}", recipes);

    let encoded = include_str!("../blueprint.b64");
    match decode_blueprint(encoded) {
        Err(e) => eprintln!("Error decoding: {}", e),
        Ok(decoded) => {
            println!("Decoded:\n{}", decoded.render());
            let bugged = check_for_bugs(decoded);
            match bugged.len() {
                0 => println!("Bug freedom achieved!"),
                _ => println!(
                    "Found {} possible bug vulnerabilities: {:?}",
                    bugged.len(),
                    bugged
                ),
            }
        }
    }
}
