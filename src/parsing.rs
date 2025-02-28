use base64::{Engine as _, engine::general_purpose::STANDARD};
use flate2::read::ZlibDecoder;
use std::{fmt::Debug, io::Read};
use serde_json::Value;

use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::entities::{Entity, TransportBelt};

pub fn decode_blueprint(input: &str) -> Result<Vec<Entity>, Box<dyn std::error::Error>> {
    // Remove the first character (equivalent to cut -c2-)
    let trimmed = input.chars().skip(1).collect::<String>();
    
    // Decode base64
    let decoded = STANDARD.decode(trimmed)?;
    
    // Decompress using zlib
    let mut decoder = ZlibDecoder::new(&decoded[..]);
    let mut result = String::new();
    decoder.read_to_string(&mut result)?;

    // Deserialize the JSON string into a serde_json::Value
    let json_value: Value = serde_json::from_str(&result)?;
    let json_entities = json_value.get("blueprint").unwrap().get("entities").unwrap();
    
    println!("{:?}", json_entities);
    for json_ent in json_entities.as_array().unwrap() {
        println!("{:?}", json_ent);
    }
        
    
    let entities: Vec<Entity> = serde_json::from_value(json_entities.clone()).unwrap();
    
    Ok(entities)
}

pub fn get_recipes_map() -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
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
            if let Some(recipe) = item_data.get("recipe")
                .and_then(|r| r.as_object())
                .and_then(|r| r.get("ingredients"))
                .and_then(|i| i.as_array()) {
                
                // Skip empty ingredient arrays
                if recipe.is_empty() {
                    continue;
                }
                
                // Extract ingredient IDs
                let mut input_ingredients = Vec::new();
                for ingredient in recipe {
                    if let Some(id) = ingredient.get("id").and_then(|id| id.as_str()) {
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