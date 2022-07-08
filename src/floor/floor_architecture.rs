use rand::prelude::*;
use rand_pcg::Pcg64;

use crate::room::math::Position;

use crate::direction::Direction3D;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FloorLayout {
    pub rooms: Vec<FloorRoom>,
    pub floor: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FloorRoom {
    pub coords: RoomCoordinates,
    pub exits: Vec<Direction3D>,
    pub stair_up: bool,
    pub stair_down: bool,
}

impl Default for FloorRoom {
    fn default() -> Self {
        Self {
            coords: RoomCoordinates::default(),
            exits: vec![],
            stair_down: false,
            stair_up: false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct RoomCoordinates {
    pub col: i32,
    pub row: i32,
}

impl RoomCoordinates {
    pub fn from_position(pos: Position) -> Self {
        Self {
            row: pos.row,
            col: pos.col,
        }
    }

    pub fn clone_delta_row(&self, delta: i32) -> Self {
        Self {
            row: self.row + delta,
            col: self.col,
        }
    }

    pub fn clone_delta_col(&self, delta: i32) -> Self {
        Self {
            row: self.row,
            col: self.col + delta,
        }
    }
}

/// A floor layout is structure of connected rooms
pub fn create_floor_layout(
    floor_size: u8,
    floor_number: i32,
    rng: &mut Pcg64,
    start_room: Position,
) -> FloorLayout {
    let coords = randomized_layout(floor_size, rng, start_room);
    let room_config = get_exits(&coords);

    let rooms: Vec<FloorRoom> = room_config
        .into_iter()
        .map(|conf| FloorRoom {
            coords: *conf.0,
            exits: conf.1,
            stair_up: false,
            stair_down: false,
        })
        .collect();

    FloorLayout {
        rooms,
        floor: floor_number,
    }
}

/// Will "dig" randomly from a start point, until floor size is reached
fn randomized_layout(
    floor_size: u8,
    rng: &mut Pcg64,
    start_room: Position,
) -> Vec<RoomCoordinates> {
    let mut pos = RoomCoordinates::from_position(start_room);
    let mut layout = vec![pos.clone()];

    while layout.len() < (floor_size as usize) {
        match rng.gen_range(0..4) {
            0 => pos.col += 1,
            1 => pos.col -= 1,
            2 => pos.row += 1,
            3 => pos.row -= 1,
            _ => (),
        }

        if !layout.contains(&pos) {
            layout.push(pos.clone());
        }
    }

    layout
}

/// Checks rooms for adjoined rooms and sets exit directions accordingly
fn get_exits(all_coords: &Vec<RoomCoordinates>) -> Vec<(&RoomCoordinates, Vec<Direction3D>)> {
    let mut result = vec![];
    for coords in all_coords {
        let mut exits = vec![];
        if all_coords.contains(&coords.clone_delta_col(1)) {
            exits.push(Direction3D::Right);
        }

        if all_coords.contains(&coords.clone_delta_col(-1)) {
            exits.push(Direction3D::Left);
        }

        if all_coords.contains(&coords.clone_delta_row(1)) {
            exits.push(Direction3D::Top);
        }

        if all_coords.contains(&coords.clone_delta_row(-1)) {
            exits.push(Direction3D::Bottom);
        }

        result.push((coords, exits));
    }

    result
}

#[cfg(test)]
mod test {
    use rand::prelude::*;
    use rand_pcg::Pcg64;

    use super::*;

    #[test]
    fn randomizes_layout_of_ajointed_rooms_and_calculates_furthest_room() {
        // arrange
        let mut rng = Pcg64::seed_from_u64(1);

        // act
        let result = create_floor_layout(8, 0, &mut rng, Position::new(0, 0));

        // assert
        assert_eq!(8, result.rooms.len());
        let coords: Vec<RoomCoordinates> = result.rooms.iter().map(|r| r.coords).collect();
        assert_eq!(
            vec![
                RoomCoordinates { col: 0, row: 0 },
                RoomCoordinates { col: 0, row: 1 },
                RoomCoordinates { col: -1, row: 1 },
                RoomCoordinates { col: 0, row: 2 },
                RoomCoordinates { col: 0, row: 3 },
                RoomCoordinates { col: -1, row: 3 },
                RoomCoordinates { col: -2, row: 3 },
                RoomCoordinates { col: -2, row: 4 }
            ],
            coords
        );
        assert_eq!(
            vec![Direction3D::Left, Direction3D::Top, Direction3D::Bottom],
            result.rooms[1].exits // col0/row1
        );
        assert_eq!(
            vec![Direction3D::Right, Direction3D::Left],
            result.rooms[5].exits // col-1/row3
        );
        assert_eq!(0, result.floor);
    }
}
