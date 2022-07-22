use rand::Rng;
use rand_pcg::Pcg64;

use crate::{direction::Direction3D, floor::floor_architecture::FloorRoom};

use super::{
    math::{Dimension, Rect},
    room::DungeonRoom,
    room_builder::RoomBuilder,
    tile::DungeonTile,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Alignment {
    Vertically,
    Horizontally,
}

pub struct GridRoomBuilder {
    pub rect_size: Dimension,
    pub rects: Dimension,
}

impl Default for GridRoomBuilder {
    fn default() -> Self {
        Self {
            rect_size: Dimension::new(3, 3),
            rects: Dimension::new(3, 3),
        }
    }
}

impl RoomBuilder for GridRoomBuilder {
    fn create_room(&self, rng: &mut Pcg64, room_config: &FloorRoom) -> DungeonRoom {
        let exits = &room_config.exits;
        let rects = self.create_rects();
        let mut room = self.room_from_rects(rng, &rects);
        self.set_exits(&mut room, exits, &rects);
        room.pathing();
        room.stair_down = room_config.stair_down;
        room.stair_up = room_config.stair_up;

        room
    }

    fn get_rows(&self) -> i32 {
        (self.rect_size.vertical * self.rects.vertical + self.rects.vertical + 1) as i32
    }

    fn get_cols(&self) -> i32 {
        (self.rect_size.horizontal * self.rects.horizontal + self.rects.horizontal + 1) as i32
    }
}

impl GridRoomBuilder {
    fn create_rects(&self) -> Vec<Rect> {
        let mut rects = vec![];
        for row in 0..(self.rects.vertical as usize) {
            let next_row_position = 1 + (row * (self.rect_size.vertical + 1));
            for col in 0..(self.rects.horizontal as usize) {
                let next_col_position = 1 + (col * (self.rect_size.horizontal + 1));
                rects.push(Rect::new(
                    next_row_position as i32,
                    (next_row_position + self.rect_size.vertical - 1) as i32,
                    next_col_position as i32,
                    (next_col_position + self.rect_size.horizontal - 1) as i32,
                ));
            }
        }

        rects
    }

    fn room_from_rects(&self, rng: &mut Pcg64, rects: &Vec<Rect>) -> DungeonRoom {
        let mut room = DungeonRoom {
            tiles: vec![DungeonTile::Wall; (self.get_cols() * self.get_rows()) as usize],
            columns: self.get_cols(),
            rows: self.get_rows(),
            ..Default::default()
        };

        self.fill(&mut room, rects);
        self.connect(&mut room, rects, rng);

        room
    }

    fn fill(&self, room: &mut DungeonRoom, rects: &Vec<Rect>) {
        for rect in rects {
            for col in rect.cols() {
                for row in rect.rows() {
                    let idx = room.room_idx(row as i32, col as i32);
                    room.tiles[idx] = DungeonTile::Floor;
                }
            }
        }
    }

    fn connect(&self, room: &mut DungeonRoom, rects: &Vec<Rect>, rng: &mut Pcg64) {
        let align = match rng.gen_range(0..2) {
            0 => Alignment::Vertically,
            _ => Alignment::Horizontally,
        };

        let doorways = self.find_doorway_connections(rng, &align);

        let h = self.rects.horizontal;
        let v = self.rects.vertical;
        for (idx, rect) in rects.iter().enumerate() {
            if (align == Alignment::Horizontally && idx % h != h - 1)
                || (align == Alignment::Vertically && doorways.contains(&idx))
            {
                let room_idx = room.room_idx(rect.center().row, rect.col2 + 1);
                room.tiles[room_idx] = DungeonTile::Floor;
                match rng.gen_range(0..4) {
                    0 => {
                        for i in 0..self.rect_size.vertical {
                            let room_idx = room.room_idx(rect.row1 + i as i32, rect.col2 + 1);
                            room.tiles[room_idx] = DungeonTile::Floor
                        }
                    }
                    _ => {}
                }
            }

            if (align == Alignment::Vertically && idx < h * (v - 1))
                || (align == Alignment::Horizontally && doorways.contains(&idx))
            {
                let room_idx = room.room_idx(rect.row2 + 1, rect.center().col);
                room.tiles[room_idx] = DungeonTile::Floor;
                match rng.gen_range(0..4) {
                    0 => {
                        for i in 0..self.rect_size.horizontal {
                            let room_idx = room.room_idx(rect.row2 + 1, rect.col1 + i as i32);
                            room.tiles[room_idx] = DungeonTile::Floor
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn find_doorway_connections(&self, rng: &mut Pcg64, align: &Alignment) -> Vec<usize> {
        let doorways_amount = match align {
            Alignment::Vertically => self.rects.vertical - 1,
            Alignment::Horizontally => self.rects.horizontal - 1,
        };

        let mut result = vec![];

        for i in 0..doorways_amount {
            let selection = self.possible_doorway_rects(align, i);
            let doorway_idx = rng.gen_range(0..selection.len());
            result.push(selection[doorway_idx]);
        }

        result
    }

    fn possible_doorway_rects(&self, align: &Alignment, iteration: usize) -> Vec<usize> {
        match *align {
            Alignment::Horizontally => {
                let current = iteration * self.rects.horizontal;
                let amount = self.rects.horizontal;
                (current..current + amount).collect()
            }
            Alignment::Vertically => (iteration..self.total_rects())
                .step_by(self.rects.vertical)
                .collect(),
        }
    }

    fn total_rects(&self) -> usize {
        self.rects.horizontal * self.rects.vertical
    }

    fn set_exits(&self, room: &mut DungeonRoom, exits: &Vec<Direction3D>, rects: &Vec<Rect>) {
        for direction in exits {
            let rect = self.side_center_rect(*direction, rects);
            let room_idx = match direction {
                Direction3D::Top => room.room_idx(rect.row1 - 1, rect.center().col),
                Direction3D::Bottom => room.room_idx(rect.row2 + 1, rect.center().col),
                Direction3D::Left => room.room_idx(rect.center().row, rect.col1 - 1),
                Direction3D::Right => room.room_idx(rect.center().row, rect.col2 + 1),
                _ => 0,
            };

            room.tiles[room_idx] = DungeonTile::Exit;
        }
    }

    // Finds the rectangle at the center of one side within the given rectangles
    fn side_center_rect(&self, direction: Direction3D, rects: &Vec<Rect>) -> Rect {
        let side_rects = self.side_rects(direction, rects);
        side_rects[side_rects.len() / 2]
    }

    // Finds all rectangles to one side of the given rectangles
    fn side_rects(&self, direction: Direction3D, rects: &Vec<Rect>) -> Vec<Rect> {
        match direction {
            Direction3D::Top => rects.iter().filter(|r| r.row1 == 1).map(|r| *r).collect(),
            Direction3D::Bottom => rects
                .iter()
                .filter(|r| r.row2 == self.get_rows() - 2)
                .map(|r| *r)
                .collect(),
            Direction3D::Left => rects.iter().filter(|r| r.col1 == 1).map(|r| *r).collect(),
            Direction3D::Right => rects
                .iter()
                .filter(|r| r.col2 == self.get_cols() - 2)
                .map(|r| *r)
                .collect(),
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::room::{math::Position, print::print_room};

    use super::*;
    use rand::prelude::*;

    #[test]
    fn creates_dungeon_room() {
        let mut rng = Pcg64::seed_from_u64(1);
        let sut = GridRoomBuilder::default();
        let room_config = FloorRoom {
            exits: vec![Direction3D::Bottom, Direction3D::Right],
            ..Default::default()
        };

        let room = sut.create_room(&mut rng, &room_config);

        let expected_tiles = "#############
#...........#
#...........#
#...........#
##.###.###.##
#...#...#...#
#...#...#...E
#...#...#...#
##.###.###.##
#...#...#...#
#...#...#...#
#...#...#...#
######E######"
            .to_string();
        let room_tile_str = print_room(room.rows as usize, room.columns as usize, room.tiles, 0, 0);
        assert_eq!(expected_tiles, room_tile_str);
    }

    #[test]
    fn creates_dungeon_room_2() {
        let mut rng = Pcg64::seed_from_u64(2);
        let sut = GridRoomBuilder {
            rects: Dimension::new(7, 7),
            ..Default::default()
        };
        let room_config = FloorRoom {
            exits: vec![Direction3D::Top, Direction3D::Left],
            ..Default::default()
        };

        let room = sut.create_room(&mut rng, &room_config);

        let expected_tiles = "##############E##############
#...#...#...#...#...#...#...#
#...#...#...#...#...#...#...#
#...#...#...#...#...#...#...#
##.###.##...##.###.###.###.##
#...#...#...#...#...#...#...#
#...#...#...#...#...#...#...#
#...#...#...#...#...#...#...#
##.###.###.###.###.###.##...#
#...#...#...#...#...#...#...#
#...#...#...#...#...#.......#
#...#...#...#...#...#...#...#
#...##.###.##...##.###.###.##
#...#...#...#...#...#...#...#
E.......#.......#...#...#...#
#...#...#...#...#...#...#...#
#...#...##.###.##...##.###.##
#...#...#...#...#...#...#...#
#...#...#...#.......#...#...#
#...#...#...#...#...#...#...#
##.###.##...##.###.###.###.##
#...#...#...#...#...#...#...#
#...#...#...#...#...#...#...#
#...#...#...#...#...#...#...#
##.###.###.###.###.###.###.##
#...#...#...#...#...#...#...#
#...#.......#...#.......#...#
#...#...#...#...#...#...#...#
#############################"
            .to_string();
        let room_tile_str = print_room(room.rows as usize, room.columns as usize, room.tiles, 0, 0);
        assert_eq!(expected_tiles, room_tile_str);
    }

    #[test]
    fn find_possible_vertical_doorway_rects() {
        let sut = GridRoomBuilder::default();
        let result = sut.possible_doorway_rects(&Alignment::Vertically, 1);
        let expect = vec![1, 4, 7];
        assert_eq!(expect, result);
    }

    #[test]
    fn find_possible_horizontal_doorway_rects() {
        let sut = GridRoomBuilder::default();
        let result = sut.possible_doorway_rects(&Alignment::Horizontally, 1);
        let expect = vec![3, 4, 5];
        assert_eq!(expect, result);
    }

    #[test]
    fn find_doorway_connections_of_vertically_aligned_rooms() {
        let mut rng = Pcg64::seed_from_u64(1);
        let sut = GridRoomBuilder::default();

        let doorway_rects = sut.find_doorway_connections(&mut rng, &Alignment::Vertically);

        let expected: Vec<usize> = vec![6, 1];
        assert_eq!(expected, doorway_rects);
    }

    #[test]
    fn find_doorway_connections_of_horizontally_aligned_rooms() {
        let mut rng = Pcg64::seed_from_u64(1);
        let sut = GridRoomBuilder::default();

        let doorway_rects = sut.find_doorway_connections(&mut rng, &Alignment::Horizontally);

        let expected: Vec<usize> = vec![2, 3];
        assert_eq!(expected, doorway_rects);
    }

    #[test]
    fn side_rects_of_all_directions() {
        let sut = GridRoomBuilder::default();
        let rects = sut.create_rects();
        assert_eq!(3, sut.side_rects(Direction3D::Top, &rects).len());
        assert_eq!(3, sut.side_rects(Direction3D::Bottom, &rects).len());
        assert_eq!(3, sut.side_rects(Direction3D::Left, &rects).len());
        assert_eq!(3, sut.side_rects(Direction3D::Right, &rects).len());
    }

    #[test]
    fn side_center_rects_of_all_directions() {
        let sut = GridRoomBuilder::default();
        let rects = sut.create_rects();

        assert_eq!(
            Position::new(10, 6),
            sut.side_center_rect(Direction3D::Bottom, &rects).center()
        );
        assert_eq!(
            Position::new(2, 6),
            sut.side_center_rect(Direction3D::Top, &rects).center()
        );
        assert_eq!(
            Position::new(6, 2),
            sut.side_center_rect(Direction3D::Left, &rects).center()
        );
        assert_eq!(
            Position::new(6, 10),
            sut.side_center_rect(Direction3D::Right, &rects).center()
        );
    }
}
