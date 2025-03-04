use crate::entities::{Entity, Blueprint};

pub fn check_for_bugs(bp: Blueprint) -> Vec<Entity> {

    let (min_x, min_y, max_x, max_y) = get_borders(bp);
    vec![]
}


fn get_borders(bp: Blueprint) -> (f64, f64, f64, f64) {
    
    bp.entities.into_iter().flat_map(|entity| entity.get_positions()).fold((f64::MAX, f64::MAX, f64::MIN, f64::MIN), |(min_x, min_y, max_x, max_y), pos| {
        (min_x.min(pos.x), min_y.min(pos.y), max_x.max(pos.x), max_y.max(pos.y))
    })

}


