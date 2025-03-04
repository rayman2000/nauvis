use base64::prelude::*;
use flate2::read::ZlibDecoder;
use serde_json::Value;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Read;

#[allow(unused_imports)]
use crate::entities::{Entity, TransportBelt};

pub fn decode_blueprint(input: &str) -> Result<Vec<Entity>, Box<dyn Error>> {
    // Trim leading and trailing spaces.
    let input = input.trim();

    // Strip the version byte [1]
    // [1]: https://wiki.factorio.com/Blueprint_string_format
    // TODO: This reallocates.
    let trimmed: Vec<u8> = input.bytes().skip(1).collect::<Vec<u8>>();

    // Decode the base64 representation.
    let decoded = BASE64_STANDARD
        .decode(trimmed)
        .expect("input should be valid base64");

    // Decompress using zlib.
    let mut decoder = ZlibDecoder::new(&decoded[..]);
    let mut inflated = String::new();
    decoder
        .read_to_string(&mut inflated)
        .expect("base 64 should be valid");

    let full_blueprint: Value = serde_json::from_str(&inflated)?;
    let entities = full_blueprint
        .get("blueprint")
        .unwrap()
        .get("entities")
        .unwrap();

    let entities: Vec<Entity> = serde_json::from_value(entities.clone())
        .expect("entities should be valid");

    Ok(entities)
}

type Recipies = HashMap<String, Vec<String>>;
pub fn get_recipes_map() -> Result<Recipies, Box<dyn Error>> {
    // Read the file content
    let content = fs::read_to_string("recipes.json")?;

    // Parse JSON
    let recipes_data: Value = serde_json::from_str(&content)?;

    // Create the ingredient map
    let mut ingredient_map: HashMap<String, Vec<String>> = HashMap::new();

    // Iterate through each item in the recipes
    if let Some(data) = recipes_data.as_object() {
        for (item_id, item_data) in data {
            // Check if the item has a recipe with ingredients
            if let Some(recipe) = item_data
                .get("recipe")
                .and_then(|r| r.as_object())
                .and_then(|r| r.get("ingredients"))
                .and_then(|i| i.as_array())
            {
                // Skip empty ingredient arrays
                if recipe.is_empty() {
                    continue;
                }

                // Extract ingredient IDs
                let mut input_ingredients = Vec::new();
                for ingredient in recipe {
                    if let Some(id) =
                        ingredient.get("id").and_then(|id| id.as_str())
                    {
                        input_ingredients.push(id.to_string());
                    }
                }

                // Only add items with actual ingredients
                if !input_ingredients.is_empty() {
                    ingredient_map.insert(item_id.clone(), input_ingredients);
                }
            }
        }
    }

    Ok(ingredient_map)
}
