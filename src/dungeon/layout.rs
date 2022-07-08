use std::ops::Range;

use crate::floor::floor_architecture::FloorLayout;

use super::{coords::{DungeonCoordinates, StairCoordinates}};

#[derive(Clone, Debug)]
pub struct DungeonLayoutConfig {
    pub floors_above: Range<u8>,
    pub floors_below: Range<u8>,
    pub floor_size: Range<u8>,
}

impl Default for DungeonLayoutConfig {
    fn default() -> Self {
        Self {
            floor_size: 3..5,
            floors_above: 0..2,
            floors_below: 0..2,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DungeonLayout {
    pub coords: Vec<DungeonCoordinates>,
    pub floors: Vec<FloorLayout>,
    pub stairs: Vec<StairCoordinates>,
    pub first_room: DungeonCoordinates,
    pub last_room: DungeonCoordinates,
}

impl Default for DungeonLayout {
    fn default() -> Self {
        Self {
            coords: vec![],
            floors: vec![],
            stairs: vec![],
            first_room: DungeonCoordinates::default(),
            last_room: DungeonCoordinates::default(),
        }
    }
}