//use std::collections::HashSet;

use crate::entities::{Blueprint, Entity, EntityType, Position};

pub fn check_for_bugs(bp: Blueprint) -> Vec<Entity> {

    let (min_x, min_y, max_x, max_y) = get_borders(&bp);

    
    let mut queue: Vec<Position> = vec![];

    // TODO this should be a hashset of some sort => BTreeSet
    let mut visited: Vec<Position> = vec![];
    let mut unsafe_entities: Vec<Entity> = vec![];

    // Add all border fields to queue and visited. Also add the positions on step outside to visited
    for x in min_x..(max_x - 1) {
        let pos_bot = Position:: new(x as f64 + 0.5, min_y as f64 + 0.5);
        let pos_top = Position:: new(x as f64 + 0.5, max_y as f64 - 0.5);
        queue.push(pos_top);
        queue.push(pos_bot);
        visited.push(pos_top);
        visited.push(pos_top.above());
        visited.push(pos_bot);
        visited.push(pos_bot.below());
    }
    for y in (min_y+1)..(max_y-2) {
        let pos_left = Position:: new(min_x as f64 + 0.5, y as f64 + 0.5);
        let pos_right = Position:: new(max_x as f64 - 0.5, y as f64 + 0.5);
        queue.push(pos_left);
        queue.push(pos_right);
        visited.push(pos_left);
        visited.push(pos_left.left());
        visited.push(pos_right);
        visited.push(pos_right.right());
    }

    // While queue non-empty
    while let Some(pos) = queue.pop() {

        match bp.entity_at(&pos) {
            None => {
                // If there is no entity, we just process the neighbours
                for n in pos.neighbours() {
                    if !visited.contains(&n) {
                        queue.push(n);
                        visited.push(n);
                    }
                }
            }
            Some(ent) => {
                match ent.entity_type(){
                    // For a wall, do nothing
                    EntityType::StoneWall {  } => {},
                    _ => {
                        // Otherwise, we found an unprotected entity and process its neighbours.
                        if !unsafe_entities.contains(&ent) {
                            unsafe_entities.push(ent.clone());
                        }
                
                        for n in pos.neighbours() {
                            if !visited.contains(&n) {
                                queue.push(n);
                                visited.push(n);
                            }
                        }
                    },
                }

            },
            
        }
    }

    unsafe_entities
}


fn get_borders(bp: &Blueprint) -> (i32, i32, i32, i32) {
    
    let (min_x, min_y, max_x, max_y) = bp.entities.iter().flat_map(|entity| entity.get_positions()).fold((f64::MAX, f64::MAX, f64::MIN, f64::MIN), |(min_x, min_y, max_x, max_y), pos| {
        (min_x.min(pos.x), min_y.min(pos.y), max_x.max(pos.x), max_y.max(pos.y))
    });
    (min_x.floor() as i32, min_y.floor() as i32, max_x.ceil() as i32, max_y.ceil() as i32)

}


