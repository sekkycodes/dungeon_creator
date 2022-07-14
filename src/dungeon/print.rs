use crate::room::print::print_room;

use super::{coords::DungeonCoordinates, room::ArrangedDungeonRoom};

pub fn print_dungeon_map(rooms: Vec<ArrangedDungeonRoom>) -> String {
    let mut output = String::new();

    let (min_floor, max_floor) =
        get_min_max_floors(&rooms.iter().map(|r| r.dungeon_coords).collect());
    println!("min floor {} - max floor {}", min_floor, max_floor);

    for f in min_floor..max_floor + 1 {
        println!("generating output for floor {}", f);
        let floor_rooms: Vec<&ArrangedDungeonRoom> = rooms
            .iter()
            .filter(|r| r.dungeon_coords.floor == f)
            .collect();
        let floor_map = print_floor_map(floor_rooms);
        output.push_str(&floor_map);
        output.push('\n');
        output.push('\n');
    }

    output
}

pub fn print_floor_map(rooms: Vec<&ArrangedDungeonRoom>) -> String {
    let mut output = String::new();

    let room_spacing = calculate_room_spacing(
        rooms
            .iter()
            .map(|r| RoomCoords {
                rows: r.rows,
                cols: r.columns,
                coords: r.dungeon_coords,
            })
            .collect(),
    );

    output.push_str(&empty_line(0));
    for room in rooms {
        output.push('\n');

        let room_output = print_room(
            room.rows as usize,
            room.columns as usize,
            room.tiles.clone(),
        );

        for room_line in room_output.split('\n') {
            if room_line.trim() == "" {
                break;
            }

            output.push(' ');
            output.push_str(room_line);
            output.push(' ');
            output.push('\n');
        }
    }
    output.push_str(&empty_line(0));

    output
}

fn calculate_room_spacing(rooms: Vec<RoomCoords>) -> Vec<Vec<RoomSpacing>> {
    let mut max_row = 0;
    let mut max_col = 0;

    for room in rooms.clone() {
        if room.coords.row > max_row {
            max_row = room.coords.row;
        }
        if room.coords.col > max_col {
            max_col = room.coords.col;
        }
    }

    println!("max_col {}", max_col);
    println!("max_row {}", max_row);

    let mut spacing =
        vec![vec![RoomSpacing::default(); (max_col + 1) as usize]; (max_row + 1) as usize];
    let mut rows_largest = vec![0; (max_col + 1) as usize];
    let mut cols_largest = vec![0; (max_row + 1) as usize];

    for room in rooms {
        let cur_row = room.coords.row as usize;
        let cur_col = room.coords.col as usize;

        if room.rows > rows_largest[cur_col] {
            rows_largest[cur_col] = room.rows;
        }
        if room.cols > cols_largest[cur_row] {
            cols_largest[cur_row] = room.cols;
        }

        spacing[room.coords.row as usize][room.coords.col as usize] = RoomSpacing {
            rows: room.rows,
            cols: room.cols,
            padding_left: 0,
            padding_top: 0,
        }
    }

    for (cur_row, row) in spacing.iter_mut().enumerate() {
        for (cur_col, spacing) in row.iter_mut().enumerate() {
            spacing.padding_left = cols_largest[cur_col] - spacing.cols;
            spacing.padding_top = rows_largest[cur_row] - spacing.rows;
        }
    }

    spacing
}

fn adjust_spacing(
    spacing: &mut Vec<Vec<RoomSpacing>>,
    cols_largest: Vec<i32>,
    rows_largest: Vec<i32>,
) {
    for (cur_row, row) in spacing.iter_mut().enumerate() {
        for (cur_col, spacing) in row.iter_mut().enumerate() {
            spacing.padding_left = cols_largest[cur_col] - spacing.cols;
            spacing.padding_top = rows_largest[cur_row] - spacing.rows;
        }
    }
}

fn get_min_max_floors(coords: &Vec<DungeonCoordinates>) -> (i32, i32) {
    let mut min_floor = 0;
    let mut max_floor = 0;

    for coord in coords {
        let room_floor = coord.floor;

        if room_floor > max_floor {
            max_floor = room_floor;
        }

        if room_floor < min_floor {
            min_floor = room_floor;
        }
    }

    (min_floor, max_floor)
}

fn empty_line(cols: i32) -> String {
    String::from_utf8(vec![b' '; (cols + 2) as usize]).expect("Failed to UTF8 string")
}

#[derive(Clone, Copy)]
struct RoomCoords {
    cols: i32,
    rows: i32,
    coords: DungeonCoordinates,
}

#[derive(Clone, Copy, Default)]
struct RoomSpacing {
    cols: i32,
    rows: i32,
    padding_left: i32,
    padding_top: i32,
}

#[cfg(test)]
pub mod test {
    use crate::{dungeon::coords::DungeonCoordinates, room::tile::DungeonTile};

    use super::*;

    #[test]
    fn prints_empty_for_dungeon_without_rooms() {
        let result = print_dungeon_map(vec![]);
        assert_eq!("", result);
    }

    #[test]
    fn prints_single_dungeon_room() {
        let rooms = vec![create_room(3)];

        let result = print_dungeon_map(rooms);
        assert_eq!("     \n ... \n ... \n ... \n     ", result);
    }

    #[test]
    fn calculates_min_and_max_floor() {
        let coords = vec![
            DungeonCoordinates {
                floor: 1,
                ..Default::default()
            },
            DungeonCoordinates {
                floor: -2,
                ..Default::default()
            },
        ];

        let (min, max) = get_min_max_floors(&coords);

        assert_eq!(-2, min);
        assert_eq!(1, max);
    }

    #[test]
    fn print_dungeon_with_one_floor_with_several_rooms() {
        let mut top_room = create_room(3);
        top_room.dungeon_coords = DungeonCoordinates {
            col: 1,
            row: 0,
            floor: 0,
        };

        let mut left_room = create_room(3);
        left_room.dungeon_coords = DungeonCoordinates {
            col: 0,
            row: 1,
            floor: 0,
        };

        let mut center_room = create_room(3);
        center_room.dungeon_coords = DungeonCoordinates {
            col: 1,
            row: 1,
            floor: 0,
        };

        let rooms = vec![top_room, left_room, center_room];

        let result = print_dungeon_map(rooms);

        let expected =
            "           \n      ... \n      ... \n      ... \n           \n ... ... \n ... ... \n ... ... \n           ";

        assert_eq!(expected, result);
    }

    #[test]
    fn calculates_room_spacing() {
        let room_coords = vec![create_room_coords(3, 0, 0), create_room_coords(3, 1, 0)];

        let result = calculate_room_spacing(room_coords);

        assert_eq!(2, result.len());
        assert_eq!(0, result[0][0].padding_left);
        assert_eq!(0, result[1][0].padding_left);
    }

    #[test]
    fn calculates_room_spacing_different_room_sizes() {
        let room_coords = vec![create_room_coords(3, 0, 0), create_room_coords(5, 1, 0)];

        let result = calculate_room_spacing(room_coords);

        assert_eq!(2, result.len());
        assert_eq!(0, result[0][0].padding_left);
        assert_eq!(0, result[0][0].padding_top);
        assert_eq!(2, result[1][0].padding_left);
        assert_eq!(2, result[1][0].padding_top);
    }

    #[test]
    fn adjusts_room_spacing_by_setting_paddings() {
        let mut spacing = vec![vec![RoomSpacing::default(); 2]; 2];

        spacing[1][1].rows = 5;
        spacing[1][1].cols = 5;
        spacing[1][0].rows = 3;
        spacing[1][0].cols = 3;

        adjust_spacing(&mut spacing, vec![0, 5], vec![3, 5]);

        println!(
            "paddings; left: {}, top: {}",
            spacing[0][0].padding_left, spacing[0][0].padding_top
        );

        assert_eq!(0, spacing[0][0].padding_left);
        assert_eq!(3, spacing[0][0].padding_top);
        assert_eq!(5, spacing[0][1].padding_left);
        assert_eq!(0, spacing[0][1].padding_top);
    }

    fn create_room(size: usize) -> ArrangedDungeonRoom {
        ArrangedDungeonRoom {
            tiles: vec![DungeonTile::Floor; size * size],
            pathing: vec![],
            rows: size as i32,
            columns: size as i32,
            dungeon_coords: DungeonCoordinates {
                ..Default::default()
            },
            entry: Option::None,
            exits: vec![],
            rotation: 0,
            stair_up: false,
            stair_down: false,
        }
    }

    fn create_room_coords(size: i32, row: i32, col: i32) -> RoomCoords {
        RoomCoords {
            cols: size,
            rows: size,
            coords: DungeonCoordinates { floor: 0, col, row },
        }
    }
}
