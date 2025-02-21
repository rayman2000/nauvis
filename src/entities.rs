use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBelt {
    position: Position,
    direction: i32,
    entity_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum Entity {

    #[serde(rename = "transport-belt")]
    TransportBelt(TransportBelt),

}