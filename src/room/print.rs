use super::tile::DungeonTile;

pub fn print_room(
    rows: usize,
    cols: usize,
    tiles: Vec<DungeonTile>,
    vertical_padding: usize,
    horizontal_padding: usize,
) -> String {
    let mut output = String::new();

    let empty_line = " ".repeat(rows + horizontal_padding * 2);
    for _ in 0..vertical_padding {
        output.push_str(&empty_line);
        output.push('\n');
    }

    let padding = " ".repeat(horizontal_padding);
    for r in 0..rows {
        let row_str: String = tiles
            .iter()
            .skip(r * cols)
            .take(cols)
            .map(|t| {
                if *t == DungeonTile::Floor {
                    '.'
                } else if *t == DungeonTile::Wall {
                    '#'
                } else if *t == DungeonTile::Exit {
                    'E'
                } else if *t == DungeonTile::StairsDown {
                    'v'
                } else if *t == DungeonTile::StairsUp {
                    '^'
                } else {
                    '?'
                }
            })
            .collect();

        output.push_str(&padding);
        output.push_str(&row_str);
        output.push_str(&padding);
        output.push('\n');
    }

    for _ in 0..vertical_padding {
        output.push_str(&empty_line);
        output.push('\n');
    }

    if output.ends_with('\n') {
        output.pop();
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::room::{room::DungeonRoom, tile::DungeonTile};

    #[test]
    fn prints_empty_room() {
        let room = DungeonRoom {
            tiles: vec![DungeonTile::Floor; 9],
            columns: 3,
            rows: 3,
            ..Default::default()
        };

        let result = print_room(room.rows as usize, room.columns as usize, room.tiles, 0, 0);

        println!("{}", result);
        assert_eq!("...\n...\n...", result);
    }

    #[test]
    fn prints_room_tiles() {
        let room = DungeonRoom {
            tiles: vec![
                DungeonTile::Wall,
                DungeonTile::Exit,
                DungeonTile::Wall,
                DungeonTile::Floor,
                DungeonTile::StairsUp,
                DungeonTile::StairsDown,
                DungeonTile::Wall,
                DungeonTile::Wall,
                DungeonTile::Wall,
            ],
            columns: 3,
            rows: 3,
            ..Default::default()
        };

        let result = print_room(room.rows as usize, room.columns as usize, room.tiles, 0, 0);

        println!("{}", result);
        assert_eq!("#E#\n.^v\n###", result);
    }

    #[test]
    fn prints_room_with_padding() {
        let room = DungeonRoom {
            tiles: vec![DungeonTile::Floor; 9],
            columns: 3,
            rows: 3,
            ..Default::default()
        };

        let result = print_room(room.rows as usize, room.columns as usize, room.tiles, 1, 2);

        println!("{}", result);
        assert_eq!("       \n  ...  \n  ...  \n  ...  \n       ", result);
    }
}
