use amethyst::ecs::{
    Component, 
    DenseVecStorage,
    Entity,
};
use pathfinding::prelude::{
    astar,
    absdiff,
};
use crate::components::Map;

const DIAGONAL_COST_1: i32 = 1;
const DIAGONAL_COST_2: i32 = 1;
const STRAIGHT_COST: i32 = 1;

fn heuristic_distance(a: &PathNode, b: &PathNode, allow_diagonals: bool) -> i32 {
    let dx = absdiff(a.x, b.x);
    let dy = absdiff(a.y, b.y);
    if allow_diagonals {
        DIAGONAL_COST_1 * (dx + dy) + (DIAGONAL_COST_2 - 2 * DIAGONAL_COST_1) * dx.min(dy)
    } else {
        STRAIGHT_COST * (dx + dy)
    }
}

fn distance(a: &PathNode, b: &PathNode, allow_diagonals: bool) -> f32 {
    let dx = (a.x as f32 - b.x as f32).abs();
    let dy = (a.y as f32 - b.y as f32).abs();

    if allow_diagonals {
        (dx*dx + dy*dy).sqrt()
    } else {
        dx + dy
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct PathNode {
    pub x: i32,
    pub y: i32,
}

impl PathNode {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Path contains the working data for pathfinding
pub struct Path {
    pub map_entity: Entity,
    objective: PathNode,
    origin: PathNode,
    pub path: Option<(Vec<PathNode>, i32)>,
}

impl Path {
    pub fn new(map_entity: Entity, objective: PathNode, origin: PathNode) -> Self {
        Self {
            map_entity,
            objective,
            origin,
            path: None,
        }
    }
    pub fn run(&mut self, map: &Map) {
        if self.path.is_none() {
            self.path = astar(
                &self.origin,
                |p| map.successors((p.x as usize, p.y as usize)).into_iter().map(|((x, y), cost)| (PathNode::new(x as i32, y as i32), cost)),
                |p| heuristic_distance(p, &self.objective, true),
                |p| *p == self.objective,
            );
        }
    }
}

impl Component for Path {
    type Storage = DenseVecStorage<Self>;
}