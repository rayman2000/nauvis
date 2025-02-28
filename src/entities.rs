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
    #[serde(rename = "filter-inserter")]
    FilterInserter(FilterInserter),
    #[serde(rename = "assembling-machine-1")]
    AssemblingMachine(AssemblingMachine),
    #[serde(rename = "electric-furnace")]
    ElectricFurnace(ElectricFurnace),
    #[serde(rename = "underground-belt")]
    UndergroundBelt(UndergroundBelt),
    #[serde(rename = "chemical-plant")]
    ChemicalPlant(ChemicalPlant),
    #[serde(rename = "splitter")]
    Splitter(Splitter),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssemblingMachine {
    position: Position,
    direction: i32,
    entity_number: i32,
    recipe: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectricFurnace {
    position: Position,
    direction: i32,
    entity_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UndergroundBelt {
    position: Position,
    direction: i32,
    entity_number: i32,
    #[serde(rename = "type")]
    belt_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChemicalPlant {
    position: Position,
    direction: i32,
    entity_number: i32,
    recipe: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Splitter {
    position: Position,
    direction: i32,
    entity_number: i32,
    filter: String,
    input_priority: String,
    output_priority: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterInserter {
    position: Position,
    direction: i32,
    entity_number: i32,
    filters: Vec<Filter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    index: i32,
    name: String,
}

