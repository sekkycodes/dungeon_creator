use rand::prelude::*;
use rand_pcg::Pcg64;

use crate::{direction::Direction3D, floor::floor_architecture::FloorRoom};

use super::{room::DungeonRoom, room_builder::RoomBuilder, tile::DungeonTile};

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    FindExits,
    ReverseCenter,
}

#[derive(Clone, Debug)]
pub struct DrunkardRoomBuilder {
    pub rows: usize,
    pub cols: usize,
    pub iterations: u8,
    pub steps: u8,
    pub mode: Mode,
}

impl RoomBuilder for DrunkardRoomBuilder {
    fn create_room(&self, rng: &mut Pcg64, room_config: &FloorRoom) -> DungeonRoom {
        let default_tile_type = match self.mode {
            Mode::FindExits => DungeonTile::Wall,
            Mode::ReverseCenter => DungeonTile::Floor,
        };
        let mut room = DungeonRoom {
            tiles: vec![default_tile_type; (self.rows * self.cols) as usize],
            rows: self.rows,
            columns: self.cols,
            stair_up: room_config.stair_up,
            stair_down: room_config.stair_down,
            ..Default::default()
        };

        let mut next_start_point = (self.rows / 2, self.cols / 2);

        let mut exits_hit: Vec<Direction3D> = vec![];
        let mut all_exits_hit = false;
        let mut iters = 0;

        // the digger needs to hit exits on all relevant sides; it continues to dig until it has dug out to every side we need
        while !all_exits_hit || iters < self.iterations {
            iters += 1;
            self.drunkard(next_start_point, rng, &mut room);
            exits_hit = self.get_hit_exits(&room);
            next_start_point =
                self.calculate_next_start_point(&room, &exits_hit, &room_config.exits);
            all_exits_hit = room_config.exits.iter().all(|e| exits_hit.contains(e));
        }

        // close unwanted exits: the drunk digger may have accidentally knocked open an exit on an unwanted side of the room
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

impl DrunkardRoomBuilder {
    fn drunkard(&self, start: (usize, usize), rng: &mut Pcg64, room: &mut DungeonRoom) {
        let next_start = start.clone();
        let mut drunkard_pos = (next_start.0 as i32, next_start.1 as i32);
        let mut distance_staggered = 0;
        let dug_tile = match self.mode {
            Mode::FindExits => DungeonTile::Floor,
            Mode::ReverseCenter => DungeonTile::Wall,
        };

        loop {
            if drunkard_pos.0 < 0 || drunkard_pos.1 < 0 {
                panic!("dunkard has negative coordinates")
            }

            let drunk_idx = room.room_idx(drunkard_pos.0 as usize, drunkard_pos.1 as usize);
            room.tiles[drunk_idx] = dug_tile;
            match rng.gen_range(0..4) {
                0 => drunkard_pos.0 -= 1,
                1 => drunkard_pos.0 += 1,
                2 => drunkard_pos.1 -= 1,
                _ => drunkard_pos.1 += 1,
            }

            if drunkard_pos.0 < 0 || drunkard_pos.1 < 0 {
                break;
            }

            if !room.in_bounds(drunkard_pos.0, drunkard_pos.1)
                || room.is_corner(drunkard_pos.0 as usize, drunkard_pos.1 as usize)
            {
                // difficult to handle corner exits, since they could point in 2 directions; avoid this case
                break;
            }

            distance_staggered += 1;
            if distance_staggered > self.steps {
                break;
            }
        }
    }

    fn get_hit_exits(&self, room: &DungeonRoom) -> Vec<Direction3D> {
        let mut directions = vec![];
        for (row, col) in room
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == DungeonTile::Floor)
            .map(|(idx, _)| (room.row(idx), room.col(idx)))
        {
            if row == self.rows - 1 && !directions.contains(&Direction3D::Top) {
                directions.push(Direction3D::Top);
            }

            if row == 0 && !directions.contains(&Direction3D::Bottom) {
                directions.push(Direction3D::Bottom);
            }

            if col == self.cols - 1 && !directions.contains(&Direction3D::Right) {
                directions.push(Direction3D::Right);
            }

            if col == 0 && !directions.contains(&Direction3D::Left) {
                directions.push(Direction3D::Left);
            }
        }

        return directions;
    }

    fn calculate_next_start_point(
        &self,
        room: &DungeonRoom,
        exits_hit: &Vec<Direction3D>,
        exits_to_hit: &Vec<Direction3D>,
    ) -> (usize, usize) {
        let center = (self.rows / 2, self.cols / 2);
        if self.mode == Mode::ReverseCenter {
            return center;
        }

        let direction_not_hit = exits_to_hit.iter().find(|e| !exits_hit.contains(e));
        if let Some(direction) = direction_not_hit {
            let floors = room
                .tiles
                .iter()
                .enumerate()
                .filter(|(_, t)| **t == DungeonTile::Floor)
                .map(|(idx, _)| idx);
            let result = match direction {
                Direction3D::Top => floors.max_by_key(|idx| room.row(*idx)),
                Direction3D::Bottom => floors.min_by_key(|idx| room.row(*idx)),
                Direction3D::Left => floors.min_by_key(|idx| room.col(*idx)),
                Direction3D::Right => floors.max_by_key(|idx| room.col(*idx)),
                _ => None,
            };

            return match result {
                None => center,
                Some(r) => (room.row(r), room.col(r)),
            };
        }

        // fallback: start from center
        return center;
    }
}

#[cfg(test)]
mod test {
    use crate::room::print::print_room;

    use super::*;

    #[test]
    fn find_closest_tile_to_upper_exit() {
        // arrange
        let mut sut = create_sut();
        sut.rows = 5;
        sut.cols = 5;
        let mut room = DungeonRoom {
            tiles: vec![DungeonTile::Wall; 25],
            rows: 5,
            columns: 5,
            ..Default::default()
        };
        room.tiles[7] = DungeonTile::Floor;
        room.tiles[12] = DungeonTile::Floor;
        room.tiles[11] = DungeonTile::Floor;
        let exits_hit: Vec<Direction3D> = Vec::new();

        // act
        let result = sut.calculate_next_start_point(&room, &exits_hit, &vec![Direction3D::Top]);

        // assert
        assert_eq!((2, 2), result);
    }

    #[test]
    fn defaults_to_center_if_no_exit_needs_to_be_hit() {
        // arrange
        let mut sut = create_sut();
        sut.rows = 5;
        sut.cols = 5;
        let mut room = DungeonRoom {
            tiles: vec![DungeonTile::Wall; 25],
            rows: 5,
            columns: 5,
            ..Default::default()
        };
        room.tiles[7] = DungeonTile::Floor;
        room.tiles[12] = DungeonTile::Floor;
        room.tiles[11] = DungeonTile::Floor;
        let exits_hit: Vec<Direction3D> = Vec::new();
        let exits_to_hit: Vec<Direction3D> = Vec::new();

        // act
        let result = sut.calculate_next_start_point(&room, &exits_hit, &exits_to_hit);

        // assert
        assert_eq!((2, 2), result);
    }

    #[test]
    fn defaults_to_center_if_no_floor_tiles_can_be_found() {
        // arrange
        let mut sut = create_sut();
        sut.rows = 5;
        sut.cols = 5;
        let room = DungeonRoom {
            tiles: vec![DungeonTile::Wall; 25],
            rows: 5,
            columns: 5,
            ..Default::default()
        };
        let exits_hit: Vec<Direction3D> = Vec::new();
        let exits_to_hit: Vec<Direction3D> = vec![Direction3D::Top];

        // act
        let result = sut.calculate_next_start_point(&room, &exits_hit, &exits_to_hit);

        // assert
        assert_eq!((2, 2), result);
    }

    #[test]
    fn creates_room_with_tiles_equal_to_row_times_columns() {
        let mut rng = Pcg64::seed_from_u64(1);
        let mut sut = create_sut();
        sut.rows = 15;
        sut.cols = 15;
        let room_config = FloorRoom {
            exits: vec![Direction3D::Bottom],
            ..Default::default()
        };

        let result = sut.create_room(&mut rng, &room_config);

        assert_eq!(result.tiles.len(), 225);
        assert_eq!(result.columns, 15);
        assert_eq!(result.rows, 15);
    }

    #[test]
    fn creates_printable_room() {
        let mut rng = Pcg64::seed_from_u64(1);
        let mut sut = create_sut();
        sut.rows = 15;
        sut.cols = 15;
        let room_config = FloorRoom {
            exits: vec![Direction3D::Bottom],
            ..Default::default()
        };

        let result = sut.create_room(&mut rng, &room_config);
        let output = print_room(result.rows, result.columns, result.tiles, 0, 0);

        assert_eq!(
            "#.#############
#..############
#..############
##.############
#..############
#..############
#..############
#.......#######
####...########
###############
###############
###############
###############
###############
###############",
            output
        );
    }

    #[test]
    fn eliminates_wall_tiles() {
        let mut rng = Pcg64::seed_from_u64(1);
        let sut = create_sut();
        let room_config = FloorRoom {
            exits: vec![],
            ..Default::default()
        };

        let result = sut.create_room(&mut rng, &room_config);

        let floor_count = result
            .tiles
            .iter()
            .filter(|t| **t == DungeonTile::Floor)
            .count();

        assert!(floor_count > 0);
    }

    fn create_sut() -> DrunkardRoomBuilder {
        DrunkardRoomBuilder {
            cols: 3,
            rows: 3,
            iterations: 2,
            steps: 2,
            mode: Mode::FindExits,
        }
    }
}
