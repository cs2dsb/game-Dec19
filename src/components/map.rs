use amethyst::ecs::{Component, DenseVecStorage};


use amethyst::{
	ecs::prelude::*,
	core::math::{
		Vector2,
		Vector3,
	}
};
use rand::{
	self,
	Rng,
	rngs::ThreadRng,
	seq::SliceRandom,
};
use ndarray::{
	Array2,
	Axis,
	s,
};
use crate::config::{
		Map as MapConfig,
		map::{
				Room as RoomConfig,
				MapEntity as MapEntityConfig,
		},
};
use std::slice::IterMut;

union Transmute<T: Copy, U: Copy> {
		from: T,
		to: U,
}

//This transmute hack is because nalgebra doesn't support const fn constructors for now
const NEIGHBOUR_UP: Vector2<i8> = unsafe { Transmute::<[i8; 2], Vector2<i8>> { from: [0, -1] }.to };
const NEIGHBOUR_DOWN: Vector2<i8> = unsafe { Transmute::<[i8; 2], Vector2<i8>> { from: [0, 1] }.to };
const NEIGHBOUR_LEFT: Vector2<i8> = unsafe { Transmute::<[i8; 2], Vector2<i8>> { from: [-1, 0] }.to };
const NEIGHBOUR_RIGHT: Vector2<i8> = unsafe { Transmute::<[i8; 2], Vector2<i8>> { from: [1, 0] }.to };
const NEIGHBOUR_UP_LEFT: Vector2<i8> = unsafe { Transmute::<[i8; 2], Vector2<i8>> { from: [-1, -1] }.to };
const NEIGHBOUR_UP_RIGHT: Vector2<i8> = unsafe { Transmute::<[i8; 2], Vector2<i8>> { from: [1, -1] }.to };
const NEIGHBOUR_DOWN_LEFT: Vector2<i8> = unsafe { Transmute::<[i8; 2], Vector2<i8>> { from: [-1, 1] }.to };
const NEIGHBOUR_DOWN_RIGHT: Vector2<i8> = unsafe { Transmute::<[i8; 2], Vector2<i8>> { from: [1, 1] }.to };
const NEIGHBOURS_4: [Vector2<i8>; 4] = [
		NEIGHBOUR_UP,
		NEIGHBOUR_DOWN,
		NEIGHBOUR_LEFT,
		NEIGHBOUR_RIGHT,
];
const NEIGHBOURS_8: [Vector2<i8>; 8] = [
		NEIGHBOUR_UP,
		NEIGHBOUR_DOWN,
		NEIGHBOUR_LEFT,
		NEIGHBOUR_RIGHT,
		NEIGHBOUR_UP_LEFT,
		NEIGHBOUR_UP_RIGHT,
		NEIGHBOUR_DOWN_LEFT,
		NEIGHBOUR_DOWN_RIGHT,
];


#[derive(Debug, Copy, Clone, PartialEq)]
enum TileState {
	Wall,
	Door,
	Corridor(u32),
	Room(u32),
}
use self::TileState::*;

impl Default for TileState {
	fn default() -> Self {
		Wall
	}
}

#[derive(Debug)]
pub struct MapObject {
	//Start and end are indexes into the map state array
	start: Vector2<usize>,
	end: Vector2<usize>,
	//Pos and size are world coordinates
	pos: Vector3<f32>,
	size: Vector3<f32>,
	entity: Option<Entity>,
}

impl MapObject {
	fn new(start: Vector2<usize>, end: Vector2<usize>, config: &MapEntityConfig) -> Self {
		let unit_size = config.unit_size;
		//The map is in xy but the real world the map in in the xz plane with a constant height in y
		let size = Vector3::new(
			(end.x - start.x + 1) as f32 * unit_size,
			config.wall_height * unit_size,
			(end.y - start.y + 1) as f32 * unit_size,
		);
		let pos = Vector3::new(
			start.x as f32 * unit_size,
			0.0,
			start.y as f32 * unit_size,
		);
		Self {
			start,
			end,
			pos,
			size,
			entity: None,
		}
	}

    pub fn start(&self) -> &Vector2<usize> { &self.start }
    pub fn end(&self) -> &Vector2<usize> { &self.end }

	pub fn pos(&self) -> &Vector3<f32> {
		&self.pos
	}

	pub fn size(&self) -> &Vector3<f32> {
		&self.size
	}

	pub fn entity(&self) -> &Option<Entity> {
		&self.entity
	}

	pub fn set_entity(&mut self, entity: Entity) {
		if self.entity.is_some() {
			panic!("Attempted to set MapObject.entity twice");
		}
		self.entity = Some(entity);
	}

	pub fn remove_entity(&mut self) -> Entity {
		self.entity.take().expect("Attempted to remove MapObject.entity but it was none")
	}

	fn sq_distance_point(&self, point: &Vector3<f32>) -> f32 {
		let min = self.pos;
		let max = self.pos + self.size;

		let mut dist = 0.0;
		for d in 0..3 {
			let v = point[d];
			if v < min[d] {
				dist += (min[d] - v).powf(2.0);
			} else if v > max[d] {
				dist += (v - max[d]).powf(2.0);
			}
		}
		dist
	}

	pub fn in_radius(&self, centre: &Vector3<f32>, radius: f32) -> bool {
		let dist = self.sq_distance_point(centre);
		let r2 = radius.powf(2.0);
		dist <= r2
	}
}

#[derive(Debug, Copy, Clone)]
struct Door {
	index: (usize, usize), //Position in the state array
	separates: (TileState, TileState), //What is on either side of this door
}

impl PartialEq for Door {
	fn eq(&self, other: &Self) -> bool {
		(self.separates.0 == other.separates.0 && self.separates.1 == other.separates.1) ||
		(self.separates.0 == other.separates.1 && self.separates.1 == other.separates.0)
	}
}

pub struct Map {
	width: usize,
	height: usize,
	state: Array2<TileState>,
	//These are only for early debugging visualisation
	rooms: Vec<MapObject>,
	doors: Vec<MapObject>,
	corridors: Vec<MapObject>,
	floors: Vec<MapObject>,
	walls: Vec<MapObject>,
}

impl Component for Map {
		type Storage = DenseVecStorage<Self>;
}

impl Map {
	pub fn new(width: u32, height: u32) -> Self {
		let width = width as usize;
		let height = height as usize;
		Self {
			width,
			height,
			state: Array2::default((width, height)),
			rooms: Vec::new(),
			doors: Vec::new(),
			corridors: Vec::new(),
			floors: Vec::new(),
			walls: Vec::new(),
		}
	}

	pub fn width(&self) -> u32 { self.width as u32}
	pub fn height(&self) -> u32 { self.height as u32 }
	pub fn is_wall(&self, x: u32, y: u32) -> bool {
		let i = (x as usize, y as usize);
		self.state[i] == Wall
	}

	//These are just for early visualization
	#[allow(dead_code)]
	pub fn rooms(&self) -> &[MapObject] { &self.rooms }
	#[allow(dead_code)]
	pub fn doors(&self) -> &[MapObject] { &self.doors }
	#[allow(dead_code)]
	pub fn corridors(&self) -> &[MapObject] { &self.corridors }
	#[allow(dead_code)]
	pub fn floors(&self) -> &[MapObject] { &self.floors }
	#[allow(dead_code)]
	pub fn floors_mut(&mut self) -> IterMut<MapObject> {
		self.floors.iter_mut()
	}
	#[allow(dead_code)]
	pub fn walls(&self) -> &[MapObject] { &self.walls }
	#[allow(dead_code)]
	pub fn walls_mut(&mut self) -> IterMut<MapObject> {
		self.walls.iter_mut()
	}


	fn print(&self) {
        for x in self.state.axis_iter(Axis(0)).rev() {
            println!();
            for y in x.iter().rev() {
                match y {
                    Room(_) => print!(" "),
                    Corridor(_) => print!(" "),
                    Door => print!("d"),
                    _ => print!("#"),
                }
            }
        }
		println!();
	}

	fn reset_state(&mut self) {
		for s in self.state.iter_mut() {
			*s = Default::default();
		}
		self.rooms.clear();
		self.doors.clear();
		self.corridors.clear();
		self.floors.clear();
		self.walls.clear();
	}

	fn place_room(&mut self, room_size: &RoomConfig, entity_config: &MapEntityConfig, rng: &mut ThreadRng, room_id: u32) -> Result<(), ()> {
		let size_x = rng.gen_range(room_size.min_size.0 as usize, room_size.max_size.0 as usize);
		let size_y = rng.gen_range(room_size.min_size.1 as usize, room_size.max_size.1 as usize);

		if (size_x + 2) >= self.width || (size_y + 2) >= self.height {
			return Err(());
		}

		let room_x = rng.gen_range(1, self.width - (size_x + 1));
		let room_y = rng.gen_range(1, self.height - (size_y + 1));

		//Needs a buffer of 1 on all sides
		let start_x = room_x - 1;
		let start_y = room_y - 1;
		let end_x = room_x + size_x + 1;
		let end_y = room_y + size_y + 1;

		//Check all tiles are free
		for s in self.state.slice(s![start_x..end_x, start_y..end_y]).iter() {
			if *s != Default::default() {
				return Err(())
			}
		}

		//Mark tiles in the room
		for s in self.state.slice_mut(s![room_x..(room_x + size_x), room_y..(room_y + size_y)]).iter_mut() {
			*s = Room(room_id);
		}

		//Add a room object
		self.rooms.push(MapObject::new(
			Vector2::new(room_x, room_y),
			Vector2::new(room_x + size_x - 1, room_y + size_y - 1),
			entity_config));

		log::debug!("Placed {} room ({}, {}) at ({}, {})", room_size.name, size_x, size_y, room_x, room_y);
		Ok(())
	}

	fn fill_rooms(&mut self, config: &MapConfig, rng: &mut ThreadRng) {
		let room_sizes = &config.room_sizes;
		for rs in room_sizes.iter() {
			log::info!("Room Size: {}, ({}, {}) -> ({}, {})",
				rs.name, rs.min_size.0, rs.min_size.1, rs.max_size.0, rs.max_size.1);
		}

		let mut room_id = 0;
		let mut fail_count = 0;
		let mut room_chances = vec![0.0; room_sizes.len()];
		while fail_count < config.room_place_max_iterations {
			for i in 0..room_sizes.len() {
				room_chances[i] += room_sizes[i].frequency;

				if room_chances[i] > 1.0 {
					room_chances[i] -= 1.0;

					match self.place_room(&room_sizes[i], &config.entity, rng, room_id) {
						Ok(_) => room_id += 1,
						Err(_) => fail_count += 1,
					};
				}
			}
		}

		log::info!("Placed {} rooms", self.rooms.len());
	}

	fn check_neighbours(&self, (x, y): (usize, usize), neighbours: &Vec<Vector2<i8>>, are: TileState, outok: bool) -> Result<(), ()> {
		for n in neighbours.iter() {
			match self.resolve_neighbour((x, y), n) {
				Some(ni) => if self.state[ni] != are {
					return Err(());
				},
				None => if outok {
					continue;
				} else {
					return Err(());
				},
			}
		}
		Ok(())
	}

	fn resolve_neighbour(&self, (x, y): (usize, usize), neighbour: &Vector2<i8>) -> Option<(usize, usize)> {
		let ix = x as i32 + neighbour.x as i32;
		let iy = y as i32 + neighbour.y as i32;

		if ix < 0 || iy < 0 {
			return None;
		}

		let ux = ix as usize;
		let uy = iy as usize;

		if ux >= self.width || uy >= self.height {
			return None;
		}

		Some((ux, uy))
	}

	fn fill_maze_from(&mut self, start: (usize, usize), corridor_id: u32, neighbours4: &mut Vec<Vector2<i8>>, neighbours8: &Vec<Vector2<i8>>, config: &MapConfig, rng: &mut ThreadRng) {
		let new_state = Corridor(corridor_id);
		self.state[start] = new_state;

		let turn_chance = config.corridor_turn_chance;

		let mut covered_cells = 0;
		let mut backtrack = Vec::new();
		//This index tells us where to start in the neighbours4 list.
		//  It persists between runs of the outer loop so corridors tend to
		//  carry on in the same direction they were going (unless turn_chance is hit)
		let mut neighbour_i = 0;
		let mut current = start;
		let mut done = false;
		while !done {
			let mut next = None;

			if rng.gen::<f32>() < turn_chance {
				neighbours4.shuffle(rng);
			}

			//Find a new neighbour on 4 directions
			for i in 0..neighbours4.len() {
				let n = &neighbours4[(neighbour_i + i) % neighbours4.len()];
				if let Some(next_i) = self.resolve_neighbour(current, n) {
					if self.state[next_i] != Default::default() {
						continue;
					}
					//Check it's clear on all 8 directions
					let mut clear = true;
					for n in neighbours8.iter() {
						match self.resolve_neighbour(next_i, n) {
							Some(other_i) => {
								if other_i == current { //Don't check against where we came from
									continue;
								}
								if let Some(prev) = backtrack.last() {
									if other_i == *prev { //Otherwise we can't turn corners ;)
										continue;
									}
								}
								if self.state[other_i] != Default::default() {
									clear = false;
									break;
								}
							},
							None => {
								clear = false; //Hit the edge of the world (we want a 1 cell border for walls)
								break;
							}
						}
					}
					if clear {
						next = Some(next_i);
						neighbour_i = (neighbour_i + i) % neighbours4.len();
						break;
					}
				}
			}

			if let Some(next) = next { //We found a new place to go
				//Mark the new cell as part of this corridor
				self.state[next] = new_state;
				//Add the prev cell for backtracking
				backtrack.push(current);
				//Move to the new cell
				current = next;
				covered_cells += 1;
			} else if let Some(prev) = backtrack.pop() {
				//Backtrack
				current = prev;
			} else {
				//All reachable cells have been visited
				done = true;
			}
		}

		log::debug!("Corridor {} covers {} cells", corridor_id, covered_cells);
	}

	fn fill_maze(&mut self, config: &MapConfig, rng: &mut ThreadRng) {
		let neighbours8 = NEIGHBOURS_8.to_vec();
		let mut neighbours4 = NEIGHBOURS_4.to_vec();

		let mut corridor_id = 0;

		//let mut backtrack = Vec::new();
		let mut done = false;
		while !done {
			done = true;

			//Find an cell
			let mut cell = None;
			for (i, s) in self.state.indexed_iter() {
				if *s == Default::default() {
					match self.check_neighbours(i, &neighbours8, Default::default(), false) {
						Ok(_) => {
							cell = Some(i);
							break;
						},
						Err(_) => continue,
					}
				}
			}
			if let Some(i) = cell {
				//Create a maze from this point
				self.fill_maze_from(i, corridor_id, &mut neighbours4, &neighbours8, config, rng);
				corridor_id += 1;
				//Something to do, carry on
				done = false;
			}
			//log::debug!("{}, {}", x, y);
		}
	}

	fn resolve_door(&self, index: (usize, usize), a: Option<(usize, usize)>, b: Option<(usize, usize)>) -> Option<Door> {
		if let (Some(a), Some(b)) = (a, b) {
			let s_a = self.state[a];
			let s_b = self.state[b];
			let is_door = match (s_a, s_b) {
				(Corridor(_), Room(_)) | (Room(_), Corridor(_)) => true,
				(Corridor(a_id), Corridor(b_id)) | (Room(a_id), Room(b_id)) => a_id != b_id,
				_ => false,
			};
			if is_door {
				return Some(Door {
					index,
					separates: (s_a, s_b),
				});
			}
		}
		None
	}

	fn add_doors(&mut self, config: &MapConfig, rng: &mut ThreadRng) {
		let mut possible_doors = Vec::new();

		//Find all possible possible_doors
		for (i, s) in self.state.indexed_iter() {
			if *s != Wall { continue; }
			if let Some(door) = self.resolve_door(
																i,
																self.resolve_neighbour(i, &NEIGHBOUR_LEFT),
																self.resolve_neighbour(i, &NEIGHBOUR_RIGHT)) {
				possible_doors.push(door);
			}
			if let Some(door) = self.resolve_door(
																i,
																self.resolve_neighbour(i, &NEIGHBOUR_UP),
																self.resolve_neighbour(i, &NEIGHBOUR_DOWN)) {
				possible_doors.push(door);
			}
		}

		log::debug!("Found {} possible_doors", possible_doors.len());

		//Shuffle the possible_doors
		possible_doors.shuffle(rng);

		let mut extra_doors = (self.rooms.len() as f32 * config.superfluous_doors_per_room).max(0.0) as u32;

		//Select some doors. I tried this with sorting and deduping but it became a mess
		//  this implementation is naive but it is good enough for now.
		let mut doors = Vec::new();
		for d in possible_doors.iter() {
			let mut found = false;
			for o in doors.iter() {
				if d == o {
					found = true;
					break;
				}
			}
			if found && extra_doors > 0 {
				extra_doors -= 1;
				found = false;
			}
			if !found {
				doors.push(d.clone());
			}
		}

		log::debug!("Selected {} doors", doors.len());

		for d in doors.iter() {
			self.state[d.index] = Door;
		}
	}

	fn remove_dead_ends(&mut self) {
		let neighbours4 = vec![
			NEIGHBOUR_UP,
			NEIGHBOUR_DOWN,
			NEIGHBOUR_LEFT,
			NEIGHBOUR_RIGHT,
		];

		let mut removed = 0;

		for y in 0..self.height as usize {
			for x in 0..self.width as usize {
				let i = (x, y);
				let s = self.state[i];

				match s {
					Corridor(_) => {},
					_ => continue,
				};

				let mut current = Some(i);
				while let Some(c) = current {
					current = None;
					let mut walls = 0;
					let mut back_i = neighbours4.len();

					//Work out how many walls it has around it
					for (i, n) in neighbours4.iter().enumerate() {
						if let Some(n) = self.resolve_neighbour(c, n) {
							match self.state[n] {
								Wall => walls += 1,
								Corridor(_) => back_i = i,
								_ => {},
							}
						} else {
							walls += 1; //Off edge, same diff
						}
					}

					if walls >= 3 {
						removed += 1;
						self.state[c] = Wall;

						if back_i < neighbours4.len() {
							current = self.resolve_neighbour(c, &neighbours4[back_i]);
						}
					}
				}
			}
		}

		log::info!("Removed {} dead ends", removed);
	}

	fn fill_debug_vecs(&mut self, config: &MapEntityConfig) {
		for ((x, y), s) in self.state.indexed_iter() {
			match s {
				Corridor(_) => self.corridors.push(MapObject::new(Vector2::new(x, y), Vector2::new(x, y), config)),
				Door => self.doors.push(MapObject::new(Vector2::new(x, y), Vector2::new(x, y), config)),
				_ => {},
			}
		}
		let mut floor = MapObject::new(Vector2::new(0, 0), Vector2::new(self.width-1, self.height-1), config);
		floor.pos.y -= config.unit_size;
		self.floors.push(floor);
	}

	fn greedy_mesh_walls(&mut self, config: &MapEntityConfig) {
		let w = self.width as usize;
		let h = self.height as usize;

		let mut visited: Array2<bool> = Array2::default((w, h));
		let mut wall_count = 0;
		//Iterate over the whole array once
		//Currently this is just a standard greedy mesh producing rectangle regions
		for y in 0..h {
			for x in 0..w {
				let i = (x, y);
				//Skip squares we've already visited
				if visited[i] { continue; }

				//If there is no block, mark as visited and skip
				if self.state[i] != Wall {
					visited[i] = true;
					continue;
				}

				let x_start = x;
				let y_start = y;
				let mut x_end = x;
				let mut y_end = y;

				//Go as far as we can in x
				for x2 in (x_start+1)..w {
					let i = (x2, y_start);
					if visited[i] || self.state[i] != Wall {
						//Already visited or run out of wall
						break;
					}
					x_end = x2; //We're still in wall
				}

				//Go as far as we can in y
				'y_loop: for y2 in (y_start+1)..h {
					//Checking all the x's we've already stretched to
					for x2 in x_start..=x_end {
						let i = (x2, y2);
						if visited[i] || self.state[i] != Wall {
							//Already visited or run out of wall
							break 'y_loop;
						}
					}
					y_end = y2;
				}

				//Add the new section
				self.walls.push(MapObject::new(
					Vector2::new(x_start, y_start),
					Vector2::new(x_end, y_end),
					config));

				//Mark every square as visited
				for y2 in y_start..=y_end {
					for x2 in x_start..=x_end {
						visited[(x2, y2)] = true;
					}
				}

				wall_count += (x_end - x_start + 1) * (y_end - y_start + 1);
			}
		}

		log::info!("Greedy meshing reduced {} wall blocks to {} meshes", wall_count, self.walls.len());
	}

	pub fn generate(&mut self, config: &MapConfig) {
		self.reset_state();

		let mut rng = rand::thread_rng();

		//Place as many rooms as possible
		self.fill_rooms(config, &mut rng);

		//Fill the remaining space with maze
		self.fill_maze(config, &mut rng);

		//Add doors between different rooms/corridors
		self.add_doors(config, &mut rng);

		//Remove dead ends
		self.remove_dead_ends();

		self.fill_debug_vecs(&config.entity);

		self.greedy_mesh_walls(&config.entity);

		self.print();
	}

	///Pretty janky, doesn't account for radius of the from point and always gives the center of a tile back
	///just for POC. Physics would presumably take over from this when it's implemented.
	pub fn get_closest_traversable_location(&self, from: &Vector3<f32>, config: &MapConfig) -> Option<Vector3<f32>> {
		let unit_size = config.entity.unit_size;

		//First check if the cell is inside the map
		let (x, y, clamped) = clamp(from.x / unit_size, from.z / unit_size, self.width, self.height);
		if self.state[(x, y)] != Wall {
			return if !clamped {
				None
			} else {
				Some(Vector3::new(
					(x as f32 + 0.5) * unit_size,
					0.5 * unit_size,
					(y as f32 + 0.5) * unit_size
				))
			};
		}

		//Find the closest non wall point
		let mut best = None;

		let xi = x as isize;
		let yi = y as isize;
		for r in 1..(self.width.max(self.height) as isize) {
			let mut r_best_d = usize::max_value();
			let mut r_best = None;
			for s in 0..=r {
				for (xo, yo) in [
					(s, r-s),
					(-s, r-s),
					(s, -(r-s)),
					(-s, -(r-s)),
				].iter() {
					let x2 = xi + xo;
					let y2 = yi + yo;

					if !self.is_inside(x2, y2) { continue; }

					let i = (x2 as usize, y2 as usize);
					if self.state[i] == Wall { continue; }

					//No need to sqrt since we're only comparing to itself
					let d = (x-i.0).pow(2) + (y-i.1).pow(2);

					if d < r_best_d {
						r_best_d = d;
						r_best = Some(i);
					}
				}
			}
			if r_best.is_some() {
				best = r_best;
				break;
			}
		}

		if let Some((x, y)) = best {
			Some(Vector3::new(
				(x as f32 + 0.5) * unit_size,
				0.5 * unit_size,
				(y as f32 + 0.5) * unit_size
			))
		} else {
			None
		}
	}

	fn is_inside(&self, x: isize, y: isize) -> bool {
		if x < 0 || y < 0 {
			false
		} else if (x as usize) >= self.width || (y as usize) >= self.height {
			false
		} else {
			true
		}
	}
}

fn clamp(x: f32, y: f32, w: usize, h: usize) -> (usize, usize, bool) {
	let mut clamped = false;

	let x = if x < 0.0 {
		clamped = true;
		0
	} else {
		let x = x as usize;
		if x < w {
			x
		} else {
			clamped = true;
			w - 1
		}
	};

	let y = if y < 0.0 {
		clamped = true;
		0
	} else {
		let y = y as usize;
		if y < h {
			y
		} else {
			clamped = true;
			h - 1
		}
	};

	(x, y, clamped)
}