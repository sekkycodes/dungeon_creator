use std::ops::Range;

use rand::prelude::*;
use rand_pcg::Pcg64;

use crate::{room::math::Position, floor::floor_architecture::create_floor_layout};

use super::{layout::{DungeonLayoutConfig, DungeonLayout}, coords::{DungeonCoordinates, StairCoordinates}};


pub struct DungeonArchitect {
    pub config: DungeonLayoutConfig,
}

impl Default for DungeonArchitect {
    fn default() -> Self {
        Self {
            config: DungeonLayoutConfig::default(),
        }
    }
}

impl DungeonArchitect {
    pub fn create_dungeon_layout(&self, rng: &mut Pcg64) -> DungeonLayout {
        let floors_above = rng.gen_range(self.config.floors_above.clone());
        let floors_below = rng.gen_range(self.config.floors_below.clone());

        let mut layout = DungeonLayout::default();
        self.layout_floor(rng, &mut layout, Position::new(0, 0), 0);
        layout.first_room = layout.coords[0];

        // nominate stair coordinates, then build upper & lower floors
        let stair_room = find_distanced_room_on_floor(&layout, layout.first_room);
        if floors_above > 0 {
            // add stair coordinates, use stair room row/col coordinates for next room
            // iterate over floors_above and build floor layouts
            self.layout_floors(
                rng,
                &mut layout,
                Position::new(stair_room.row, stair_room.col),
                1..floors_above + 2,
                false,
            );
        }

        if floors_below > 0 {
            // add stair coordinates, use stair room row/col coordinates for next room
            // iterate over floors_below and build floor layouts
            self.layout_floors(
                rng,
                &mut layout,
                Position::new(stair_room.row, stair_room.col),
                1..floors_below + 2,
                true,
            );
        }

        // calculate last room (most distanced room from start room at 0/0/0)
        layout.last_room =
            find_distanced_room_in_dungeon(&layout, DungeonCoordinates::new(0, 0, 0));

        layout
    }

    fn layout_floors(
        &self,
        rng: &mut Pcg64,
        layout: &mut DungeonLayout,
        start_room: Position,
        floors: Range<u8>,
        negative_floors: bool,
    ) {
        let mut floor_start_room = start_room;
        let mut floor_before = 0;
        for floor_abs in floors {
            let mut floor = floor_abs as i32;
            if negative_floors {
                floor *= -1;
            }

            // add stair coordinates
            layout.stairs.push(StairCoordinates::from_coords(
                floor_start_room.row,
                floor_start_room.col,
                floor_before,
                floor,
            ));

            // layout the floor
            self.layout_floor(rng, layout, floor_start_room, floor);

            // find the most distanced room as stair room & start room for next floor
            let distanced_room = find_distanced_room_on_floor(
                layout,
                DungeonCoordinates {
                    floor,
                    col: floor_start_room.col,
                    row: floor_start_room.row,
                },
            );
            floor_before = floor;
            floor_start_room = Position::new(distanced_room.row, distanced_room.col);
        }

        set_stairs(layout);
    }

    fn layout_floor(
        &self,
        rng: &mut Pcg64,
        layout: &mut DungeonLayout,
        start_room: Position,
        floor: i32,
    ) {
        let floor_size = rng.gen_range(self.config.floor_size.clone());
        let ground_floor = create_floor_layout(floor_size, floor, rng, start_room);
        layout.floors.push(ground_floor.clone());

        for room in ground_floor.rooms {
            // add all the room coordinates from the floor layout
            layout.coords.push(DungeonCoordinates {
                floor,
                col: room.coords.col,
                row: room.coords.row,
            })
        }
    }
}

pub fn find_distanced_room_on_floor(
    layout: &DungeonLayout,
    from: DungeonCoordinates,
) -> DungeonCoordinates {
    layout
        .coords
        .iter()
        .filter(|c| c.floor == from.floor)
        .max_by_key(|c| (from.row - c.row).abs() + (from.col - c.col).abs())
        .unwrap()
        .to_owned()
}

pub fn find_distanced_room_in_dungeon(
    layout: &DungeonLayout,
    from: DungeonCoordinates,
) -> DungeonCoordinates {
    layout
        .coords
        .iter()
        .max_by_key(|c| {
            // floor is weighted *4 to prejudice towards rooms on other floors
            (from.row - c.row).abs() + (from.col - c.col).abs() + (from.floor - c.floor).abs() * 4
        })
        .unwrap()
        .to_owned()
}

fn set_stairs(layout: &mut DungeonLayout) {
    let up_rooms: Vec<DungeonCoordinates> = layout.stairs.iter().map(|s| s.lower_floor).collect();
    let down_rooms: Vec<DungeonCoordinates> = layout.stairs.iter().map(|s| s.upper_floor).collect();

    for floor in layout.floors.iter_mut() {
        for room in floor.rooms.iter_mut() {
            let dungeon_coords = DungeonCoordinates {
                floor: floor.floor,
                col: room.coords.col,
                row: room.coords.row,
            };

            room.stair_up = up_rooms.contains(&dungeon_coords);
            room.stair_down = down_rooms.contains(&dungeon_coords);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_dungeon_layout() {
        let sut = DungeonArchitect::default();
        let mut rng = Pcg64::seed_from_u64(1);

        let result = sut.create_dungeon_layout(&mut rng);

        assert_eq!(10, result.coords.len());
        assert_eq!(2, result.stairs.len());
        assert_eq!(DungeonCoordinates::new(-2, -1, 3), result.last_room);
    }

    #[test]
    fn finds_most_distanced_room_on_floor_by_coordinates() {
        let mut rng = Pcg64::seed_from_u64(1);
        let floor = create_floor_layout(7, 0, &mut rng, Position::new(0, 0));
        let layout = DungeonLayout {
            coords: floor
                .rooms
                .iter()
                .map(|f| DungeonCoordinates {
                    floor: 0,
                    row: f.coords.row,
                    col: f.coords.col,
                })
                .collect(),
            ..Default::default()
        };
        let result = find_distanced_room_on_floor(&layout, DungeonCoordinates::default());

        assert_eq!(
            DungeonCoordinates {
                floor: 0,
                col: -2,
                row: 3
            },
            result
        );
    }
}
