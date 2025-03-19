use crate::entities::{Blueprint, Entity, EntityType};



pub fn check_for_leaks(bp: &Blueprint) -> Vec<Entity> {

    let inserters = bp.entities.iter().filter(|ent| {
        matches!(ent.ty, EntityType::FilterInserter { .. })
    });

    
    vec![]

}