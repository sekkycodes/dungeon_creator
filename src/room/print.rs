use super::tile::DungeonTile;

pub fn print_room(rows: usize, cols: usize, tiles: Vec<DungeonTile>) -> String {
    let mut output = String::new();
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
                    'e'
                } else if *t == DungeonTile::StairsDown {
                    'v'
                } else if *t == DungeonTile::StairsUp {
                    '^'
                } else {
                    '?'
                }
            })
            .collect();

        output.push_str(&row_str);
        output.push('\n');
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

        let result = print_room(room.rows as usize, room.columns as usize, room.tiles);

        println!("{}", result);
        assert_eq!("...\n...\n...\n", result);
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
                DungeonTile::Exit,
                DungeonTile::Wall,
            ],
            columns: 3,
            rows: 3,
            ..Default::default()
        };

        let result = print_room(room.rows as usize, room.columns as usize, room.tiles);

        println!("{}", result);
        assert_eq!("#e#\n.^v\n#e#\n", result);
    }
}
