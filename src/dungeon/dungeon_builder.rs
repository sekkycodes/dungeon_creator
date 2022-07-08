use crate::{direction::Direction3D, map::tile::DungeonTile};

use super::{floor_architecture::FloorLayout, layout::{DungeonLayout, DungeonLayoutConfig}, dungeon_architecture::DungeonArchitect, room::ArrangedDungeonRoom, coords::DungeonCoordinates, room_builder::RoomBuilder};
use rand::prelude::*;
use rand_pcg::Pcg64;

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