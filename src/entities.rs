use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Debug, Serialize, Deserialize)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
#[serde(untagged)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    position: Position,
    direction: Direction,
    entity_number: i32,
    #[serde(flatten)]
    ty: EntityType,
}

// TODO: populate the enums here.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "kebab-case")]
enum EntityType {
    TransportBelt,
    #[serde(rename = "assembling-machine-1")]
    AssemblingMachine {},
    FilterInserter {},
    ElectricFurnace {},
    UndergroundBelt {},
    ChemicalPlant {},
    Splitter {},
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportBelt {
    position: Position,
    direction: Direction,
    entity_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblingMachine {
    position: Position,
    direction: Direction,
    entity_number: i32,
    recipe: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectricFurnace {
    position: Position,
    direction: Direction,
    entity_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UndergroundBelt {
    position: Position,
    direction: Direction,
    entity_number: i32,
    #[serde(rename = "type")]
    belt_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChemicalPlant {
    position: Position,
    direction: Direction,
    entity_number: i32,
    recipe: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Splitter {
    position: Position,
    direction: Direction,
    entity_number: i32,
    filter: String,
    input_priority: String,
    output_priority: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterInserter {
    position: Position,
    direction: Direction,
    entity_number: i32,
    filters: Vec<Filter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    index: i32,
    name: String,
}
