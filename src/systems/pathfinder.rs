use amethyst::{
    ecs::prelude::{
        Join, 
        Read, 
        ReadStorage, 
        System, 
        WriteStorage,
        Entities,
        LazyUpdate,
    },
};

use crate::components::{
    Navigator,
    Map,
    Path,
    PathNode,
};

pub struct PathFinder;

/// Create paths for entities with navigator components
fn create_paths(
    entities: &Entities,
    navigators: &ReadStorage<Navigator>,
    paths: &WriteStorage<Path>,
    maps: &ReadStorage<Map>,
    lazy_update: &LazyUpdate,
) {
    let mut add_to = Vec::new();

    // Find navigators with no path components that don't have them
    for (e, _, _) in (entities, navigators, !paths).join() {
        add_to.push(e);
    }        

    if add_to.len() > 0 {
        let mut map = None;
        let mut hack_map = None;

        // Find an active map
        for (e, m) in (entities, maps).join() {
            if entities.is_alive(e) {
                map = Some(e);
                hack_map = Some(m);
                break;
            }
        }

        // If we found a map (if not, we'll just end up here later)
        if let (Some(map), Some(hack_map)) = (map, hack_map) {
            let mut x0 = 0;
            let mut y0 = 0;

            while hack_map.is_wall(x0, y0) {
                x0 += 1;
                if x0 > hack_map.width() - 1 {
                    x0 = 0;
                    y0 += 1;
                }
            }

            let mut x1 = hack_map.width() - 1;
            let mut y1 = hack_map.height() - 1;

            while hack_map.is_wall(x1, y1) {
                if x1 == 1 {
                    x1 = hack_map.width() - 1;
                    y1 -= 1;
                } else {
                    x1 -= 1;
                }
            }

            let origin = PathNode::new(x0 as i32, y0 as i32);
            let objective = PathNode::new(x1 as i32, y1 as i32);

            for e in add_to {
                let path = Path::new(
                    map,
                    origin,
                    objective,
                );
                lazy_update.insert(e, path);
            }
        }
    }
}

/// Remove paths where the map they point to is dead
fn remove_dead_paths<'s>(
    entities: &Entities,
    mut paths: WriteStorage<'s, Path>,
) -> WriteStorage<'s, Path> {
    let mut remove_from = Vec::new();

    // Find all path components with dead maps where the map is dead
    for (e, p) in (entities, &paths).join() {
        if entities.is_alive(e) && !entities.is_alive(p.map_entity) {
            remove_from.push(e);
        }
    }

    for e in remove_from {
        paths.remove(e);
    }

    paths
}


/// Run pathfinding on path components
fn run_pathfinding(
    entities: &Entities,
    paths: &mut WriteStorage<Path>,
    maps: &ReadStorage<Map>,
) {
    for (e, p) in (entities, paths).join() {
        // No point if either owner is dead
        if !entities.is_alive(e) || !entities.is_alive(p.map_entity) {
            continue;
        }

        if let Some(map) = maps.get(p.map_entity) {
            if p.path.is_none() {
                p.run(map);
            }
        } else {
            log::error!("Map component missing for {:?} referred to by {:?}'s path component", p.map_entity, e);
        }
    }
}



impl<'s> System<'s> for PathFinder {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Navigator>,
        WriteStorage<'s, Path>,
        Read<'s, LazyUpdate>,
        ReadStorage<'s, Map>,
    );

    fn run(&mut self, (entities, navigators, mut paths, lazy_update, maps): Self::SystemData) {
        create_paths(&entities, &navigators, &paths, &maps, &lazy_update);
        paths = remove_dead_paths(&entities, paths);
        run_pathfinding(&entities, &mut paths, &maps);
    }
}