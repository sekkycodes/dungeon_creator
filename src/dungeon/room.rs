use crate::{
    direction::Direction3D,
    room::{room::DungeonRoom, tile::DungeonTile},
};

use super::coords::DungeonCoordinates;

#[derive(Clone, PartialEq, Eq)]
pub struct ArrangedDungeonRoom {
    pub tiles: Vec<DungeonTile>,
    pub pathing: Vec<usize>,
    pub rows: usize,
    pub columns: usize,
    pub dungeon_coords: DungeonCoordinates,
    pub entry: Option<(usize, Direction3D)>,
    pub exits: Vec<(usize, Direction3D)>,
    pub rotation: i8,
    pub stair_up: bool,
    pub stair_down: bool,
}

impl Default for ArrangedDungeonRoom {
    fn default() -> Self {
        Self {
            tiles: vec![],
            pathing: vec![],
            rows: 0,
            columns: 0,
            dungeon_coords: DungeonCoordinates::default(),
            entry: None,
            exits: vec![],
            rotation: 0,
            stair_up: false,
            stair_down: false,
        }
    }
}

impl ArrangedDungeonRoom {
    pub fn from(room: &DungeonRoom) -> Self {
        Self {
            columns: room.columns,
            rows: room.rows,
            tiles: room.tiles.clone(),
            pathing: room.pathing.clone(),
            entry: None,
            exits: vec![],
            dungeon_coords: DungeonCoordinates::default(),
            rotation: 0,
            stair_down: room.stair_down,
            stair_up: room.stair_up,
        }
    }

    pub fn room_idx(&self, row: usize, col: usize) -> usize {
        (row * self.columns) + col
    }

    pub fn col(&self, idx: usize) -> usize {
        idx % self.columns
    }

    pub fn row(&self, idx: usize) -> usize {
        idx / self.columns
    }

    pub fn border_path_tiles(&self, direction: Direction3D) -> Vec<usize> {
        let filter: Box<dyn FnMut(&&usize) -> bool> = match direction {
            Direction3D::Top => Box::new(|u| self.top(**u)),
            Direction3D::Bottom => Box::new(|u| self.bottom(**u)),
            Direction3D::Right => Box::new(|u| self.right(**u)),
            Direction3D::Left => Box::new(|u| self.left(**u)),
            _ => Box::new(|_| false),
        };

        self.pathing.iter().filter(filter).map(|t| *t).collect()
    }

    fn top(&self, tile: usize) -> bool {
        self.row(tile) == 0
    }

    fn bottom(&self, tile: usize) -> bool {
        self.row(tile) == self.rows - 1
    }

    fn left(&self, tile: usize) -> bool {
        self.col(tile) == 0
    }

    fn right(&self, tile: usize) -> bool {
        self.col(tile) == self.columns - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_vec_of_border_tiles() {
        let sut = ArrangedDungeonRoom {
            columns: 3,
            rows: 4,
            tiles: vec![DungeonTile::Floor; 12],
            pathing: vec![0, 1, 3, 5],
            ..Default::default()
        };

        let down = sut.border_path_tiles(Direction3D::Top);
        let left = sut.border_path_tiles(Direction3D::Left);

        assert_eq!(vec![0, 1], down);
        assert_eq!(vec![0, 3], left);
    }
}
