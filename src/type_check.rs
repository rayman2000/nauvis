#![allow(dead_code, unused_variables)]
//! We can check a blueprint for well-formedness. A blueprint is well-formed if
//! and only if the following is true:
//!   All belt sides carry at most one type of item.

use crate::entities::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BeltSide {
    Left,
    Right,
}

struct Blueprint {
    entities: Vec<Entity>,
}

struct Lanes {
    cnt: usize,
    belongs_to: BTreeMap<(Position, BeltSide), usize>,
}

impl Lanes {
    pub fn new() -> Lanes {
        Lanes {
            cnt: 0,
            belongs_to: BTreeMap::new(),
        }
    }

    pub fn compute_lanes_of(&mut self, bp: &Blueprint) {
        use EntityType::*;
        let es = bp.entities.iter().filter(|e| e.is_beltlike());
        for e in es {
            match e.ty {
                TransportBelt => {
                    let (l, r) = self.register_belt_at(e.position);
                }
                _ => (),
            }
        }
    }

    fn fresh_region(&mut self) -> usize {
        self.cnt += 1;
        self.cnt
    }

    fn register_belt_at(&mut self, p: Position) -> (usize, usize) {
        let left_region = self.fresh_region();
        let right_region = self.fresh_region();

        self.belongs_to.insert((p, BeltSide::Left), left_region);
        self.belongs_to.insert((p, BeltSide::Right), left_region);

        (left_region, right_region)
    }
}

fn compute_lanes(bp: &Blueprint) -> Lanes {
    let mut lanes = Lanes::new();

    lanes
}

struct Constraints {}

fn check_well_formedness(bp: &Blueprint) -> Result<Constraints, &'static str> {
    unimplemented!()
}

fn leak_free(bp: &Blueprint) {}
