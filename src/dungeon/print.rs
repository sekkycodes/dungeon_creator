use crate::{
    floor::grid::{FloorCell, FloorGrid},
    room::print::print_room,
};

use super::room::ArrangedDungeonRoom;

pub fn print_floor(rooms: &Vec<ArrangedDungeonRoom>) -> String {
    if rooms.len() == 0 {
        return String::new();
    }

    let grid = fill_floor_grid(rooms);

    let mut output = String::new();
    output.push('\n');

    for cur_row in 0..grid.heights.len() {
        let row_rooms = rooms
            .iter()
            .filter(|r| r.dungeon_coords.row == cur_row as i32)
            .collect();

        output.push_str(&print_floor_column(
            row_rooms,
            &grid.left_pads[cur_row],
            &grid.top_pads[cur_row],
        ));

        output.push('\n');
    }

    output
}

pub fn print_floor_column(
    rooms: Vec<&ArrangedDungeonRoom>,
    left_pads: &Vec<usize>,
    top_pads: &Vec<usize>,
) -> String {
    let height = rooms.iter().map(|r| r.rows).max().unwrap();
    let mut room_outputs = vec![String::new(); (height) as usize];
    for cur_col in 0..left_pads.len() {
        let cur_room = rooms
            .iter()
            .find(|r| r.dungeon_coords.col == cur_col as i32)
            .unwrap();

        for (idx, room_row) in print_room(
            cur_room.rows as usize,
            cur_room.columns as usize,
            cur_room.tiles.clone(),
            top_pads[cur_col],
            left_pads[cur_col],
        )
        .split('\n')
        .enumerate()
        {
            if room_outputs[idx] == "" {
                room_outputs[idx].push(' ');
            }
            room_outputs[idx].push_str(&room_row);
            room_outputs[idx].push(' ');
        }
    }

    room_outputs.join("\n")
}

pub fn fill_floor_grid(rooms: &Vec<ArrangedDungeonRoom>) -> FloorGrid {
    if rooms.len() == 0 {
        return FloorGrid::new(0, 0);
    }

    let (max_row, max_col) = get_max_dimensions(rooms);
    let mut grid = FloorGrid::new(max_row + 1, max_col + 1);

    for room in rooms {
        grid.insert(FloorCell {
            col: room.dungeon_coords.col as usize,
            row: room.dungeon_coords.row as usize,
            height: room.rows as usize,
            width: room.columns as usize,
        })
    }

    grid
}

fn get_max_dimensions(rooms: &Vec<ArrangedDungeonRoom>) -> (usize, usize) {
    let mut max_row = 0;
    let mut max_col = 0;

    for room in rooms {
        let coords = room.dungeon_coords;
        if coords.row > max_row {
            max_row = coords.row;
        }
        if coords.col > max_col {
            max_col = coords.col;
        }
    }

    (max_row as usize, max_col as usize)
}

#[cfg(test)]
pub mod test {
    use crate::{
        dungeon::{coords::DungeonCoordinates, room::ArrangedDungeonRoom},
        room::tile::DungeonTile,
    };

    use super::*;

    #[test]
    pub fn prints_empty_dungeon() {
        let rooms = vec![];

        let output = print_floor(&rooms);

        assert_eq!("", output);
    }

    #[test]
    pub fn prints_single_floor_dungeon_with_one_room() {
        let rooms = vec![create_room(0, 0, 3)];

        let output = print_floor(&rooms);

        assert_eq!("\n ... \n ... \n ... \n", output);
    }

    #[test]
    pub fn prints_single_floor_dungeon_with_multiple_rooms() {
        let rooms = vec![create_room(0, 0, 3), create_room(0, 1, 3)];

        let output = print_floor(&rooms);

        assert_eq!("\n ... ... \n ... ... \n ... ... \n", output);
    }

    #[test]
    pub fn fills_floor_grid_with_rooms() {
        let rooms = vec![
            create_room(0, 0, 3),
            create_room(0, 1, 3),
            create_room(1, 1, 5),
        ];

        let grid = fill_floor_grid(&rooms);

        assert_eq!(3, grid.heights[0][0]);
        assert_eq!(5, grid.heights[1][1]);
    }

    fn create_room(row: i32, col: i32, size: usize) -> ArrangedDungeonRoom {
        ArrangedDungeonRoom {
            columns: size as i32,
            rows: size as i32,
            dungeon_coords: DungeonCoordinates {
                row,
                col,
                ..Default::default()
            },
            tiles: vec![DungeonTile::Floor; size * size],
            ..Default::default()
        }
    }
}
