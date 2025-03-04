use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "kebab-case")]
enum EntityType {
    TransportBelt,
    #[serde(rename = "assembling-machine-1")]
    AssemblingMachine  {
        recipe: String,
    },
    FilterInserter {
        filters: Vec<Filter>,
    },
    ElectricFurnace {},
    UndergroundBelt {
        #[serde(rename = "type")]
        belt_type: String,
    },
    ChemicalPlant {
        recipe: String,
    },
    Splitter {
        filter: String,
        input_priority: String,
        output_priority: String,
    },
    StoneWall {},
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    index: i32,
    name: String,
}

impl Entity {
    fn output_positions(&self)-> Vec<Position> {
        match &self.ty {
            EntityType::TransportBelt => vec![self.position],
            EntityType::AssemblingMachine { recipe } => todo!(),
            EntityType::FilterInserter { filters } => todo!(),
            EntityType::ElectricFurnace {  } => todo!(),
            EntityType::UndergroundBelt { belt_type } => todo!(),
            EntityType::ChemicalPlant { recipe } => todo!(),
            EntityType::Splitter { filter, input_priority, output_priority } => todo!(),
            EntityType::StoneWall {  } => todo!(),
        }
    }
}

fn get_surrounding(position: Position) -> Vec<Position> {
    vec![
        
    ]
}