use rand::Rng;
use rand_pcg::Pcg64;

use crate::{direction::Direction3D, floor::floor_architecture::FloorRoom};

use super::{room::DungeonRoom, room_builder::RoomBuilder, tile::DungeonTile};

pub struct AutomataRoomBuilder {
    pub rows: usize,
    pub cols: usize,
    pub wall_percent: u8,
    pub iterations: u8,
}

impl Default for AutomataRoomBuilder {
    fn default() -> Self {
        Self {
            rows: 16,
            cols: 16,
            wall_percent: 33,
            iterations: 5,
        }
    }
}

impl RoomBuilder for AutomataRoomBuilder {
    fn create_room(&self, rng: &mut Pcg64, room_config: &FloorRoom) -> DungeonRoom {
        let mut exits_hit: Vec<Direction3D> = vec![];
        let mut all_exits_hit = false;

        let mut room = DungeonRoom::default();
        room.stair_down = room_config.stair_down;
        room.stair_up = room_config.stair_up;
        while !all_exits_hit {
            room = self.random_room(rng);
            exits_hit = self.get_hit_exits(&room);
            all_exits_hit = room_config.exits.iter().all(|e| exits_hit.contains(e));
        }

        // close unwanted exit sides
        for non_wanted_exit_direction in exits_hit.iter().filter(|e| !room_config.exits.contains(e))
        {
            room.close_side(*non_wanted_exit_direction);
        }

        room
    }

    fn get_cols(&self) -> usize {
        self.cols
    }

    fn get_rows(&self) -> usize {
        self.rows
    }
}

impl AutomataRoomBuilder {
    fn random_room(&self, rng: &mut Pcg64) -> DungeonRoom {
        let tiles = self.random_noise_map(rng);
        let mut room = DungeonRoom {
            tiles,
            rows: self.rows,
            columns: self.cols,
            ..Default::default()
        };

        for _ in 0..self.iterations {
            self.iteration(&mut room);
        }

        // close corners, because they are difficutl to deal with
        room.tiles[0] = DungeonTile::Wall;
        room.tiles[(room.columns - 1) as usize] = DungeonTile::Wall;
        room.tiles[((room.rows - 1) * room.columns) as usize] = DungeonTile::Wall;
        let len = room.tiles.len();
        room.tiles[len - 1] = DungeonTile::Wall;

        room
    }

    fn random_noise_map(&self, rng: &mut Pcg64) -> Vec<DungeonTile> {
        let mut dungeon_tiles: Vec<DungeonTile> = vec![];
        for _ in 0..(self.rows * self.cols) {
            let roll = rng.gen_range(0..100);
            if roll > self.wall_percent {
                dungeon_tiles.push(DungeonTile::Floor);
            } else {
                dungeon_tiles.push(DungeonTile::Wall);
            }
        }

        dungeon_tiles
    }

    fn count_neighbors(&self, x: usize, y: usize, room: &DungeonRoom) -> usize {
        let mut neighbors = 0;
        for iy in 0..=2 {
            for ix in 0..=2 {
                if !(ix == 1 && iy == 1)
                    && room.tiles[room.room_idx(x + ix - 1, y + iy - 1)] == DungeonTile::Wall
                {
                    neighbors += 1
                }
            }
        }

        neighbors
    }

    fn iteration(&self, room: &mut DungeonRoom) {
        let mut new_tiles = room.tiles.clone();
        for y in 1..room.rows - 1 {
            for x in 1..room.columns - 1 {
                let neighbors = self.count_neighbors(x, y, room);
                let idx = room.room_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = DungeonTile::Wall;
                } else {
                    new_tiles[idx] = DungeonTile::Floor;
                }
            }
        }

        room.tiles = new_tiles;
    }
}

#[cfg(test)]
mod test {
    use rand::SeedableRng;

    use crate::{
        direction::Direction3D,
        floor::floor_architecture::FloorRoom,
        room::{print::print_room, room::DungeonRoom, tile::DungeonTile},
    };

    use super::*;

    #[test]
    fn creates_map_with_row_and_column_size() {
        run_test(|mut fixture| {
            let room_config = FloorRoom {
                exits: vec![Direction3D::Top],
                ..Default::default()
            };
            let result = fixture.sut.create_room(&mut fixture.rng, &room_config);
            assert_eq!(result.tiles.len(), 9);
        })
    }

    #[test]
    fn creates_printable_room() {
        let sut = AutomataRoomBuilder {
            cols: 7,
            rows: 7,
            iterations: 20,
            wall_percent: 30,
            ..Default::default()
        };
        let room_config = FloorRoom {
            exits: vec![Direction3D::Top, Direction3D::Left],
            ..Default::default()
        };
        let mut rng = Pcg64::seed_from_u64(1);

        let result = sut.create_room(&mut rng, &room_config);
        let output = print_room(result.rows, result.columns, result.tiles, 0, 0);

        assert_eq!(
            "#.#..##
......#
#.###.#
.....##
#.....#
#.....#
#######",
            output
        );
    }

    #[test]
    fn iterate_over_tiles() {
        run_test(|fixture| {
            let mut room = DungeonRoom {
                tiles: vec![
                    DungeonTile::Floor,
                    DungeonTile::Wall,
                    DungeonTile::Floor,
                    DungeonTile::Wall,
                    DungeonTile::Wall, // will change
                    DungeonTile::Wall,
                    DungeonTile::Floor,
                    DungeonTile::Wall,
                    DungeonTile::Floor,
                ],
                columns: 3,
                rows: 3,
                ..Default::default()
            };
            fixture.sut.iteration(&mut room);
            assert_eq!(
                room.tiles,
                vec![
                    DungeonTile::Floor,
                    DungeonTile::Wall,
                    DungeonTile::Floor,
                    DungeonTile::Wall,
                    DungeonTile::Floor, //changed
                    DungeonTile::Wall,
                    DungeonTile::Floor,
                    DungeonTile::Wall,
                    DungeonTile::Floor,
                ]
            );
        })
    }

    fn run_test<T>(test: T) -> ()
    where
        T: FnOnce(TestFixture) -> (),
    {
        let fixture = setup();
        test(fixture);
    }

    fn setup() -> TestFixture {
        let rng = Pcg64::seed_from_u64(1);
        TestFixture {
            rng,
            sut: AutomataRoomBuilder {
                rows: 3,
                cols: 3,
                wall_percent: 0,
                iterations: 10,
            },
        }
    }

    struct TestFixture {
        pub sut: AutomataRoomBuilder,
        pub rng: Pcg64,
    }
}
