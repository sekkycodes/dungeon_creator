use crate::{
    direction::Direction3D,
    floor::floor_architecture::FloorLayout,
    room::{room_builder::RoomBuilder, tile::DungeonTile},
};

use super::{
    coords::DungeonCoordinates,
    dungeon_architecture::DungeonArchitect,
    layout::{DungeonLayout, DungeonLayoutConfig},
    room::ArrangedDungeonRoom,
};
use rand::prelude::*;
use rand_pcg::Pcg64;

/// Builder trait for creating dungeons.
/// The building result is a vector of rooms with dungeon coordinates, exits, and stairs.
///
/// The intended use is for the concrete implementation to provide a layout configuration and set the room builders for the dungeon.
pub trait DungeonBuilder {
    fn layout(&self, rng: &mut Pcg64) -> DungeonLayout {
        let architect = DungeonArchitect {
            config: self.get_layout_config(),
        };

        architect.create_dungeon_layout(rng)
    }

    fn create_dungeon_floor(
        &self,
        rng: &mut Pcg64,
        floor_layout: &FloorLayout,
    ) -> Vec<ArrangedDungeonRoom>;

    fn get_layout_config(&self) -> DungeonLayoutConfig;

    fn create_rooms(
        &self,
        rng: &mut Pcg64,
        room_builders: Vec<Box<dyn RoomBuilder>>,
        floor_layout: &FloorLayout,
    ) -> Vec<ArrangedDungeonRoom> {
        let mut rooms = vec![];
        for room_config in &floor_layout.rooms {
            let room_builder_idx = rng.gen_range(0..room_builders.len());
            let random_builder = &room_builders[room_builder_idx];
            let mut room = random_builder.create_room(rng, &room_config);
            room.pathing();
            let mut arranged = ArrangedDungeonRoom::from(&room);
            self.arrange_room(&mut arranged, &room.exit_directions);
            arranged.dungeon_coords = DungeonCoordinates {
                floor: floor_layout.floor,
                col: room_config.coords.col,
                row: room_config.coords.row,
            };
            rooms.push(arranged);
        }

        rooms
    }

    fn arrange_room(&self, room: &mut ArrangedDungeonRoom, directions: &Vec<Direction3D>) {
        self.set_exits(room, directions);
        self.set_all_stairs(room);
    }

    fn set_exits(&self, room: &mut ArrangedDungeonRoom, directions: &Vec<Direction3D>) {
        if room.tiles.contains(&DungeonTile::Exit) {
            // exits were already set by room builder - no need to set exits here
            return;
        }

        if directions.len() == 0 {
            return;
        }

        for direction in directions {
            let tiles = room.border_path_tiles(*direction);
            let center_tile = tiles[tiles.len() / 2];
            room.tiles[center_tile] = DungeonTile::Exit;
            room.exits.push((center_tile, *direction));
        }
    }

    fn set_all_stairs(&self, room: &mut ArrangedDungeonRoom) {
        let mut target_path_tile = room.pathing.len() / 3;
        if room.stair_down {
            self.set_stairs(room, DungeonTile::StairsDown, target_path_tile);
            target_path_tile *= 2;
        }
        if room.stair_up {
            self.set_stairs(room, DungeonTile::StairsUp, target_path_tile);
        }
    }

    fn set_stairs(
        &self,
        room: &mut ArrangedDungeonRoom,
        stair_tile: DungeonTile,
        target_path_tile: usize,
    ) {
        let mut target_tile = -1;
        for path_tile in room.pathing.iter().skip(target_path_tile).step_by(2) {
            if room.tiles[*path_tile] == DungeonTile::Floor {
                target_tile = *path_tile as i32;
                break;
            }
        }

        if target_tile > 0 {
            room.tiles[target_tile as usize] = stair_tile;
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::{fs, path::PathBuf};

    use crate::{dungeon::print::print_dungeon, room::automata::AutomataRoomBuilder};

    use super::*;

    #[test]
    pub fn creates_printable_dungeon() {
        let sut = DummyDungeonBuilder {};
        let mut rng = Pcg64::seed_from_u64(1);
        let rooms = sut.create_dungeon(&mut rng);

        let output = print_dungeon(rooms.iter().map(|r| r).collect());

        let expected_output = resource_file_content("dungeon_output_1.txt");
        println!("{}", output);
        assert_linewise_eq(&expected_output, &output);
    }

    fn resource_file_content(filename: &str) -> String {
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("resources/test/");
        test_file.push(filename);
        let expected_output = fs::read_to_string(test_file).expect("unable to read file");
        expected_output.replace("\r\n", "\n")
    }

    fn assert_linewise_eq(expected: &str, actual: &str) {
        let exp_lines: Vec<&str> = expected.split("\n").into_iter().collect();
        let act_lines: Vec<&str> = actual.split("\n").into_iter().collect();

        assert_eq!(
            exp_lines.len(),
            act_lines.len(),
            "expected {} lines, but found {} lines",
            exp_lines.len(),
            act_lines.len()
        );

        for idx in 0..exp_lines.len() {
            assert_eq!(
                exp_lines[idx], act_lines[idx],
                "\ndifference in line {}",
                idx
            );
        }
    }

    /// A dungeon builder implementation for testing purposes
    struct DummyDungeonBuilder {}

    impl DummyDungeonBuilder {
        pub fn create_dungeon(&self, rng: &mut rand_pcg::Pcg64) -> Vec<ArrangedDungeonRoom> {
            let layout = self.layout(rng);

            let mut all_rooms: Vec<ArrangedDungeonRoom> = vec![];
            for floor in layout.floors {
                let mut rooms =
                    self.create_rooms(rng, vec![Box::new(AutomataRoomBuilder::default())], &floor);
                all_rooms.append(&mut rooms);
            }

            all_rooms
        }
    }

    impl DungeonBuilder for DummyDungeonBuilder {
        fn create_dungeon_floor(
            &self,
            rng: &mut rand_pcg::Pcg64,
            floor_layout: &crate::floor::floor_architecture::FloorLayout,
        ) -> Vec<crate::dungeon::room::ArrangedDungeonRoom> {
            let room_builder = AutomataRoomBuilder {
                rows: 20,
                cols: 20,
                wall_percent: 40,
                iterations: 2,
            };

            self.create_rooms(rng, vec![Box::new(room_builder)], floor_layout)
        }

        fn get_layout_config(&self) -> crate::dungeon::layout::DungeonLayoutConfig {
            DungeonLayoutConfig {
                ..Default::default()
            }
        }
    }
}
