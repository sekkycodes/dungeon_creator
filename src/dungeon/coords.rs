use std::cmp::{max, min};

pub struct DungeonElement;

#[derive(Clone, Default, Copy, Debug, PartialEq, Eq)]
pub struct DungeonCoordinates {
    pub floor: i32,
    pub col: i32,
    pub row: i32,
}

impl DungeonCoordinates {
    pub fn new(floor: i32, col: i32, row: i32) -> Self {
        Self { floor, col, row }
    }
}

#[derive(Clone, Default, Copy, Debug, PartialEq)]
pub struct StairCoordinates {
    pub lower_floor: DungeonCoordinates,
    pub upper_floor: DungeonCoordinates,
}

impl StairCoordinates {
    pub fn from_coords(row: i32, col: i32, floor1: i32, floor2: i32) -> Self {
        let lower_floor = min(floor1, floor2);
        let upper_floor = max(floor1, floor2);

        StairCoordinates {
            lower_floor: DungeonCoordinates::new(lower_floor, col, row),
            upper_floor: DungeonCoordinates::new(upper_floor, col, row),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn builds_stair_coordinates_from_raw_coords() {
        let result = StairCoordinates::from_coords(1, 2, 3, 4);

        assert_eq!(DungeonCoordinates::new(3, 2, 1), result.lower_floor);
        assert_eq!(DungeonCoordinates::new(4, 2, 1), result.upper_floor);
    }

    #[test]
    fn builds_stair_coordinates_from_raw_coords_with_reversed_floors() {
        let result = StairCoordinates::from_coords(1, 2, -1, -2);

        assert_eq!(DungeonCoordinates::new(-2, 2, 1), result.lower_floor);
        assert_eq!(DungeonCoordinates::new(-1, 2, 1), result.upper_floor);
    }
}
