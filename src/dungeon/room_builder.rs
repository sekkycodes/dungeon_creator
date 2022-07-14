use rand_pcg::Pcg64;

use crate::{
    direction::Direction3D,
    floor::floor_architecture::FloorRoom,
    room::{room::DungeonRoom, tile::DungeonTile},
};

pub trait RoomBuilder {
    fn create_room(&self, rng: &mut Pcg64, room_config: &FloorRoom) -> DungeonRoom;

    fn get_rows(&self) -> i32;

    fn get_cols(&self) -> i32;

    fn get_hit_exits(&self, room: &DungeonRoom) -> Vec<Direction3D> {
        let mut directions = vec![];
        for (row, col) in room
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == DungeonTile::Floor)
            .map(|(idx, _)| (room.row(idx), room.col(idx)))
        {
            if row == self.get_rows() - 1 && !directions.contains(&Direction3D::Top) {
                directions.push(Direction3D::Top);
            }

            if row == 0 && !directions.contains(&Direction3D::Bottom) {
                directions.push(Direction3D::Bottom);
            }

            if col == self.get_cols() - 1 && !directions.contains(&Direction3D::Right) {
                directions.push(Direction3D::Right);
            }

            if col == 0 && !directions.contains(&Direction3D::Left) {
                directions.push(Direction3D::Left);
            }
        }

        return directions;
    }

    fn close_side(&self, room: &mut DungeonRoom, direction: &Direction3D) {
        let tile_idxs = room.side_indexes(direction);
        for idx in tile_idxs {
            room.tiles[idx] = DungeonTile::Wall;
        }
    }
}
