use rand_pcg::Pcg64;

use crate::{
    direction::Direction3D,
    floor::floor_architecture::FloorRoom,
    room::{room::DungeonRoom, tile::DungeonTile},
};

pub trait RoomBuilder {
    fn create_room(&self, rng: &mut Pcg64, room_config: &FloorRoom) -> DungeonRoom;

    fn get_rows(&self) -> usize;

    fn get_cols(&self) -> usize;

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
}

#[cfg(test)]
pub mod test {
    use crate::{
        direction::Direction3D,
        floor::floor_architecture::FloorRoom,
        room::{room::DungeonRoom, tile::DungeonTile},
    };

    use super::RoomBuilder;

    #[test]
    pub fn calculates_hit_exists_for_horizontal_hallway_room() {
        let builder = DummyRoomBuilder {};

        let result = builder.get_hit_exits(&create_horizontal_hallway());

        assert_eq!(2, result.len());
        assert_eq!(Direction3D::Left, result[0]);
        assert_eq!(Direction3D::Right, result[1]);
    }

    #[test]
    pub fn calculates_hit_exits_for_vertical_hallway_room() {
        let builder = DummyRoomBuilder {};

        let result = builder.get_hit_exits(&create_vertical_hallway());

        assert_eq!(2, result.len());
        assert_eq!(Direction3D::Bottom, result[0]);
        assert_eq!(Direction3D::Top, result[1]);
    }

    fn create_horizontal_hallway() -> DungeonRoom {
        DungeonRoom {
            columns: 3,
            rows: 3,
            exit_directions: vec![Direction3D::Left, Direction3D::Right],
            tiles: vec![
                DungeonTile::Wall,
                DungeonTile::Wall,
                DungeonTile::Wall,
                DungeonTile::Floor,
                DungeonTile::Floor,
                DungeonTile::Floor,
                DungeonTile::Wall,
                DungeonTile::Wall,
                DungeonTile::Wall,
            ],
            exits: vec![3, 5],
            pathing: vec![3, 4, 5],
            ..Default::default()
        }
    }

    fn create_vertical_hallway() -> DungeonRoom {
        DungeonRoom {
            tiles: vec![
                DungeonTile::Wall,
                DungeonTile::Floor,
                DungeonTile::Wall,
                DungeonTile::Wall,
                DungeonTile::Floor,
                DungeonTile::Wall,
                DungeonTile::Wall,
                DungeonTile::Floor,
                DungeonTile::Wall,
            ],
            exits: vec![1, 7],
            pathing: vec![1, 4, 7],
            rows: 3,
            columns: 3,
            exit_directions: vec![Direction3D::Top, Direction3D::Bottom],
            ..Default::default()
        }
    }

    pub struct DummyRoomBuilder {}

    impl RoomBuilder for DummyRoomBuilder {
        fn create_room(
            &self,
            _rng: &mut rand_pcg::Pcg64,
            _room: &FloorRoom,
        ) -> crate::room::room::DungeonRoom {
            create_horizontal_hallway()
        }

        fn get_rows(&self) -> usize {
            3
        }

        fn get_cols(&self) -> usize {
            3
        }
    }
}
