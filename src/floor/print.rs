use super::floor_architecture::FloorLayout;

pub fn print_floor_layout(floor_layout: &FloorLayout) -> String {
    let mut output = String::new();

    let mut rooms = floor_layout.rooms.clone();
    rooms.sort_by(|r1, r2| r1.coords.cmp(&r2.coords));

    let mut cur_col = 0;
    let mut cur_row = 0;
    for room in rooms {
        while room.coords.row > cur_row {
            output.push('\n');
            cur_row += 1;
        }
        while room.coords.col > cur_col {
            output.push(' ');
            cur_col += 1;
        }

        output.push('O');
    }

    output
}

#[cfg(test)]
mod test {
    use crate::{
        direction::Direction3D,
        floor::floor_architecture::{FloorLayout, FloorRoom, RoomCoordinates},
    };

    use super::*;

    #[test]
    pub fn prints_floor_layout() {
        let floor_layout = create_floor_layout();

        let output = print_floor_layout(&floor_layout);

        assert_eq!(" O\nOO", output);
    }

    fn create_floor_layout() -> FloorLayout {
        FloorLayout {
            floor: 1,
            rooms: vec![
                FloorRoom {
                    coords: RoomCoordinates { col: 1, row: 0 },
                    exits: vec![Direction3D::Bottom],
                    ..Default::default()
                },
                FloorRoom {
                    coords: RoomCoordinates { col: 1, row: 1 },
                    exits: vec![Direction3D::Top, Direction3D::Left],
                    ..Default::default()
                },
                FloorRoom {
                    coords: RoomCoordinates { col: 0, row: 1 },
                    exits: vec![Direction3D::Right],
                    ..Default::default()
                },
            ],
        }
    }
}
