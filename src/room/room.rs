use std::collections::HashSet;

use super::pathfinding::connected_tile_sets;
use super::tile::DungeonTile;
use crate::direction::Direction3D;

#[derive(Clone, PartialEq, Eq)]
pub struct DungeonRoom {
    pub tiles: Vec<DungeonTile>,
    pub exits: Vec<usize>,
    pub pathing: Vec<usize>,
    pub rows: i32,
    pub columns: i32,
    pub exit_directions: Vec<Direction3D>,
    pub stair_up: bool,
    pub stair_down: bool,
}

impl Default for DungeonRoom {
    fn default() -> Self {
        Self {
            rows: 0,
            columns: 0,
            tiles: vec![],
            pathing: vec![],
            exits: vec![],
            exit_directions: vec![],
            stair_up: false,
            stair_down: false,
        }
    }
}

impl DungeonRoom {
    pub fn room_idx(&self, row: i32, col: i32) -> usize {
        ((row * self.columns) + col) as usize
    }

    pub fn col(&self, idx: usize) -> i32 {
        (idx as i32) % self.columns
    }

    pub fn row(&self, idx: usize) -> i32 {
        (idx as i32) / self.columns
    }

    pub fn in_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.rows && col >= 0 && col < self.columns
    }

    pub fn is_corner(&self, row: i32, col: i32) -> bool {
        let corner_coords = vec![
            (0, 0),
            (&self.rows - 1, 0),
            (0, self.columns - 1),
            (self.rows - 1, self.columns - 1),
        ];

        corner_coords.contains(&(row, col))
    }

    pub fn side_indexes(&self, direction: &Direction3D) -> Vec<usize> {
        match direction {
            Direction3D::Top => (0..(self.rows as usize)).collect(),
            Direction3D::Bottom => {
                (self.tiles.len() - (self.columns as usize)..self.tiles.len()).collect()
            }
            Direction3D::Left => (0..self.tiles.len())
                .step_by(self.columns as usize)
                .collect(),
            Direction3D::Right => ((self.columns as usize) - 1..self.tiles.len())
                .step_by(self.columns as usize)
                .collect(),
            _ => vec![],
        }
    }

    pub fn close_side(&mut self, direction: Direction3D) {
        let tile_idxs = self.side_indexes(&direction);
        let mut changed = false;
        for idx in tile_idxs {
            if self.tiles[idx] != DungeonTile::Wall {
                self.tiles[idx] = DungeonTile::Wall;
                self.exits.retain(|x| *x != idx);
                changed = true;
            }
        }

        if changed {
            self.pathing();
        }
    }

    pub fn pathing(&mut self) {
        let connected_tiles = connected_tile_sets(self);
        self.pathing = connected_tiles
            .iter()
            .max_by(|t1, t2| t1.len().cmp(&t2.len()))
            .unwrap()
            .iter()
            .map(|t| *t)
            .collect::<Vec<usize>>();
        self.pathing.sort();

        for idx in self.pathing.clone() {
            let row = self.row(idx.clone());
            let col = self.col(idx.clone());

            if row == 0 || row == self.rows - 1 || col == 0 || col == self.columns - 1 {
                self.exits.push(idx);
            }
        }

        self.exit_directions = self.find_exit_directions();
    }

    fn find_exit_directions(&self) -> Vec<Direction3D> {
        let mut result: HashSet<Direction3D> = HashSet::new();

        for exit_tile in self.exits.iter() {
            let row = self.row(exit_tile.clone());
            let col = self.col(exit_tile.clone());

            if row == 0 {
                result.insert(Direction3D::Top);
            }

            if col == 0 {
                result.insert(Direction3D::Left);
            }

            if row == self.rows - 1 {
                result.insert(Direction3D::Bottom);
            }

            if col == self.columns - 1 {
                result.insert(Direction3D::Right);
            }
        }

        result.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn side_indexes_for_tiles_of_top_direction() {
        let sut = DungeonRoom {
            tiles: vec![DungeonTile::Floor; 25],
            columns: 5,
            rows: 5,
            ..Default::default()
        };

        let expect_up = vec![0, 1, 2, 3, 4];
        let result_up = sut.side_indexes(&Direction3D::Top);
        assert_eq!(
            expect_up, result_up,
            "Expecting upper side tiles to be {:?} and are {:?}",
            expect_up, result_up
        );
    }

    #[test]
    fn side_indexes_for_tiles_of_bottom_direction() {
        let sut = DungeonRoom {
            tiles: vec![DungeonTile::Floor; 25],
            columns: 5,
            rows: 5,
            ..Default::default()
        };

        let expected = vec![20, 21, 22, 23, 24];
        let result = sut.side_indexes(&Direction3D::Bottom);
        assert_eq!(
            expected, result,
            "Expecting bottom side tiles to be {:?} and are {:?}",
            expected, result
        );
    }

    #[test]
    fn side_indexes_for_tiles_of_left_direction() {
        let sut = DungeonRoom {
            tiles: vec![DungeonTile::Floor; 25],
            columns: 5,
            rows: 5,
            ..Default::default()
        };

        let expected = vec![0, 5, 10, 15, 20];
        let result = sut.side_indexes(&Direction3D::Left);
        assert_eq!(
            expected, result,
            "Expecting left side tiles to be {:?} and are {:?}",
            expected, result
        );
    }

    #[test]
    fn side_indexes_for_tiles_of_right_direction() {
        let sut = DungeonRoom {
            tiles: vec![DungeonTile::Floor; 25],
            columns: 5,
            rows: 5,
            ..Default::default()
        };

        let expected = vec![4, 9, 14, 19, 24];
        let result = sut.side_indexes(&Direction3D::Right);
        assert_eq!(
            expected, result,
            "Expecting right side tiles to be {:?} and are {:?}",
            expected, result
        );
    }

    #[test]
    fn finds_exit_directions() {
        let sut = DungeonRoom {
            exits: vec![0, 4, 14],
            columns: 4,
            rows: 4,
            ..Default::default()
        };

        let result = sut.find_exit_directions();

        assert!(result.contains(&Direction3D::Top));
        assert!(result.contains(&Direction3D::Left));
        assert!(result.contains(&Direction3D::Bottom));
        assert!(!result.contains(&Direction3D::Right));
    }

    #[test]
    fn calculates_pathing_and_exits() {
        // arrange
        let mut sut = DungeonRoom {
            tiles: vec![DungeonTile::Floor; 16],
            columns: 4,
            rows: 4,
            ..Default::default()
        };
        sut.tiles[1] = DungeonTile::Wall;
        sut.tiles[3] = DungeonTile::Wall;
        sut.tiles[6] = DungeonTile::Wall;
        sut.tiles[10] = DungeonTile::Wall;
        sut.tiles[14] = DungeonTile::Wall;

        // act
        sut.pathing();

        // assert
        assert!(sut.exits.contains(&0));
        assert!(sut.exits.contains(&4));
        assert!(sut.exits.contains(&8));
        assert!(!sut.exits.contains(&2));
        assert!(!sut.exits.contains(&7));
        assert!(!sut.exits.contains(&15));

        assert!(sut.pathing.contains(&5));
        assert!(!sut.pathing.contains(&2));
        assert!(!sut.pathing.contains(&11));
    }

    #[test]
    fn calculates_index_from_row_and_col() {
        let sut = build_sut();

        assert_eq!(sut.room_idx(0, 0), 0);
        assert_eq!(sut.room_idx(0, 1), 1);
        assert_eq!(sut.room_idx(1, 0), 2);
        assert_eq!(sut.room_idx(1, 1), 3);
    }

    #[test]
    fn calculates_column_of_index() {
        let sut = build_sut();

        assert_eq!(sut.col(0), 0);
        assert_eq!(sut.col(1), 1);
        assert_eq!(sut.col(3), 1);
    }

    #[test]
    fn calculates_row_of_index() {
        let sut = build_sut();

        assert_eq!(sut.row(0), 0);
        assert_eq!(sut.row(1), 0);
        assert_eq!(sut.row(3), 1);
    }

    #[test]
    fn check_out_of_bounds() {
        let sut = build_sut();

        assert_eq!(sut.in_bounds(-1, 1), false);
        assert_eq!(sut.in_bounds(sut.columns, 1), false);
        assert_eq!(sut.in_bounds(1, -1), false);
        assert_eq!(sut.in_bounds(1, sut.rows), false);
    }

    #[test]
    fn check_in_bounds() {
        let sut = build_sut();

        assert_eq!(sut.in_bounds(0, 0), true);
        assert_eq!(sut.in_bounds(sut.columns - 1, sut.rows - 1), true);
    }

    #[test]
    pub fn closes_exits_to_one_side_of_room() {
        let mut room = DungeonRoom {
            columns: 3,
            rows: 3,
            exit_directions: vec![Direction3D::Left, Direction3D::Right],
            tiles: vec![
                DungeonTile::Wall,
                DungeonTile::Wall,
                DungeonTile::Wall,
                DungeonTile::Floor,
                DungeonTile::Floor,
                DungeonTile::Floor,
                DungeonTile::Wall,
                DungeonTile::Wall,
                DungeonTile::Wall,
            ],
            exits: vec![3, 5],
            pathing: vec![3, 4, 5],
            ..Default::default()
        };

        room.close_side(Direction3D::Left);

        assert_eq!(1, room.exit_directions.len());
        assert_eq!(Direction3D::Right, room.exit_directions[0]);
        assert_eq!(DungeonTile::Wall, room.tiles[3]);
    }

    fn build_sut() -> DungeonRoom {
        DungeonRoom {
            rows: 2,
            columns: 2,
            tiles: vec![DungeonTile::Floor; 4],
            ..Default::default()
        }
    }
}
