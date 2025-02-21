use base64::{Engine as _, engine::general_purpose::STANDARD};
use flate2::read::ZlibDecoder;
use std::{fmt::Debug, io::Read};
use serde_json::Value;

mod entities;

use entities::{Entity, TransportBelt};

fn decode_blueprint(input: &str) -> Result<Vec<Entity>, Box<dyn std::error::Error>> {
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


fn main() {
    let encoded = "0eJyNkNsKwjAQRP9lnlOxtV6SR39DRHpZJJBuS5KKpfTfTVoRQQXfdjczZzY7ojQ9dVazhxqhq5Yd1GmE01cuTJz5oSMoaE8NBLhoYudtwa5rrU9KMh6TgOaa7lDpdBYg9tprWkjfHQJd64Ko5ZgRjOvVVmCASkIRcLW2VC3PuydwuHDflGRjiPibm/wG5x/gLG4//1S9HUbgRtbNluyQ5nuZ7fNMyo1MBUwRYoP6+FJP0wNyjHA5";
    match decode_blueprint(encoded) {
        Ok(decoded) => println!("Decoded: {:?}", decoded),
        Err(e) => eprintln!("Error decoding: {}", e),
    }
}
