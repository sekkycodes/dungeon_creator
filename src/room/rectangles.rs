use rand::Rng;
use rand_pcg::Pcg64;

use crate::floor::floor_architecture::FloorRoom;

use super::{math::Rect, room::DungeonRoom, room_builder::RoomBuilder, tile::DungeonTile};

#[derive(Clone, Debug)]
pub struct RectanglesRoomBuilder {
    pub rows: i32,
    pub cols: i32,
    pub granularity: Granularity,
}

impl Default for RectanglesRoomBuilder {
    fn default() -> Self {
        Self {
            rows: 16,
            cols: 16,
            granularity: Granularity::Medium,
        }
    }
}

impl RoomBuilder for RectanglesRoomBuilder {
    fn create_room(&self, rng: &mut rand_pcg::Pcg64, room_config: &FloorRoom) -> DungeonRoom {
        let mut rects = self.create_rects(rng);

        let mut room = DungeonRoom {
            tiles: vec![DungeonTile::Wall; (self.cols * self.rows) as usize],
            columns: self.cols,
            rows: self.rows,
            stair_down: room_config.stair_down,
            stair_up: room_config.stair_up,
            ..Default::default()
        };

        // add rectangles where there should be exits
        for exit in room_config.exits.iter() {
            let side_tile_idxes = room.side_indexes(&exit);
            let side_center = side_tile_idxes[side_tile_idxes.len() / 2];
            let col = room.col(side_center);
            let row = room.row(side_center);
            rects.push(Rect::new(row, row, col, col));
        }

        rects.sort_by(|r1, r2| r1.center().cmp(&r2.center()));
        self.fill_and_build_corridors(&mut room, &rects, rng);

        room
    }

    fn get_rows(&self) -> i32 {
        self.rows
    }

    fn get_cols(&self) -> i32 {
        self.cols
    }
}

impl RectanglesRoomBuilder {
    fn create_rects(&self, rng: &mut Pcg64) -> Vec<Rect> {
        if self.granularity == Granularity::Full {
            return vec![Rect::new(1, self.rows - 2, 1, self.cols - 2)];
        }

        let (min, max, number) = self
            .granularity
            .size_and_number_ranges(self.rows, self.cols);

        let mut rects = vec![];
        let mut retries = 0;
        while rects.len() < number as usize && retries < 10 {
            for _ in 0..number {
                if let Some(rect) = self.new_rect(min, max, rng, &rects) {
                    rects.push(rect);
                } else {
                    retries += 1;
                }
            }
        }

        rects
    }

    fn new_rect(
        &self,
        min: i32,
        max: i32,
        rng: &mut Pcg64,
        existing_rects: &Vec<Rect>,
    ) -> Option<Rect> {
        let rect = self.create_rect(min, max, rng);

        for existing_rect in existing_rects {
            if rect.intersect(existing_rect) {
                return None;
            }
        }

        Some(rect)
    }

    fn create_rect(&self, min: i32, max: i32, rng: &mut Pcg64) -> Rect {
        let col_size = rng.gen_range(min..max);
        let row_size = rng.gen_range(min..max);
        let placement_cols = rng.gen_range(1..self.cols - 1 - col_size);
        let placement_rows = rng.gen_range(1..self.rows - 1 - row_size);
        Rect::new(
            placement_rows,
            placement_rows + row_size,
            placement_cols,
            placement_cols + col_size,
        )
    }

    fn fill_and_build_corridors(&self, room: &mut DungeonRoom, rects: &Vec<Rect>, rng: &mut Pcg64) {
        let mut ordered_rects = rects.clone();
        ordered_rects.sort_by(|a, b| a.center().cmp(&b.center()));

        for (i, rect) in ordered_rects.iter().enumerate() {
            for row in rect.rows() {
                for col in rect.cols() {
                    let room_idx = room.room_idx(row as i32, col as i32);
                    room.tiles[room_idx] = DungeonTile::Floor;
                }
            }

            if i == 0 {
                continue;
            }

            let prev = ordered_rects[i - 1].center();
            let new = rect.center();

            if rng.gen_range(0..=1) == 1 {
                self.apply_horizontal_tunnel(room, prev.col, new.col, prev.row);
                self.apply_vertical_tunnel(room, prev.row, new.row, new.col);
            } else {
                self.apply_vertical_tunnel(room, prev.row, new.row, prev.col);
                self.apply_horizontal_tunnel(room, prev.col, new.col, new.row);
            }
        }
    }

    fn apply_vertical_tunnel(&self, room: &mut DungeonRoom, row1: i32, row2: i32, col: i32) {
        use std::cmp::{max, min};
        for row in min(row1, row2)..=max(row1, row2) {
            let idx = room.room_idx(row, col);
            room.tiles[idx as usize] = DungeonTile::Floor;
        }
    }

    fn apply_horizontal_tunnel(&self, room: &mut DungeonRoom, col1: i32, col2: i32, row: i32) {
        use std::cmp::{max, min};
        for col in min(col1, col2)..=max(col1, col2) {
            let idx = room.room_idx(row, col);
            room.tiles[idx as usize] = DungeonTile::Floor;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Granularity {
    Small,
    Medium,
    Large,
    Full,
}

impl Granularity {
    pub fn size_and_number_ranges(&self, rows: i32, cols: i32) -> (i32, i32, i32) {
        let base = rows.min(cols);
        let (min, max, number) = match self {
            Granularity::Small => (base / 8, base / 5, 8),
            Granularity::Medium => (base / 6, base / 4, 6),
            Granularity::Large => (base / 4, base / 3, 3),
            Granularity::Full => (base, base, 1),
        };

        (min.max(1), max.max(2), number)
    }
}

#[cfg(test)]
mod test {
    use crate::{direction::Direction3D, room::print::print_room};

    use super::*;
    use rand::prelude::*;
    use rand_pcg::Pcg64;

    #[test]
    fn creates_rectangle_room_with_exits() {
        let mut rng = Pcg64::seed_from_u64(1);
        let sut = RectanglesRoomBuilder {
            rows: 10,
            cols: 10,
            ..Default::default()
        };
        let room_config = FloorRoom {
            exits: vec![Direction3D::Top, Direction3D::Right],
            ..Default::default()
        };

        let room = sut.create_room(&mut rng, &room_config);

        let expected_tiles = "##....####
##......##
##..##..##
#.......##
#..###..##
#.........
#.#####..#
#......###
#......###
##########"
            .to_string();
        let room_tile_str = print_room(room.rows as usize, room.columns as usize, room.tiles, 0, 0);
        assert_eq!(expected_tiles, room_tile_str);
    }

    #[test]
    fn get_room_sizes_and_amount_by_granularity() {
        let small = Granularity::Small.size_and_number_ranges(16, 24);
        let medium = Granularity::Medium.size_and_number_ranges(24, 20);
        let large = Granularity::Large.size_and_number_ranges(24, 24);
        let full = Granularity::Full.size_and_number_ranges(15, 12);

        assert_eq!((2, 3, 8), small);
        assert_eq!((3, 5, 6), medium);
        assert_eq!((6, 8, 3), large);
        assert_eq!((12, 12, 1), full);
    }
}