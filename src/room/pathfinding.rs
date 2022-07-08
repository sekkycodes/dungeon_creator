use super::room::DungeonRoom;
use super::tile::DungeonTile;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn connected_tile_sets(room: &DungeonRoom) -> Vec<HashSet<usize>> {
    let mut connected_tile_sets: Vec<HashSet<usize>> = vec![];

    for (idx, t) in room.tiles.iter().enumerate() {
        if *t == DungeonTile::Wall {
            continue;
        }

        let mut found_existing_set = false;
        for neigh in neighbor_floors(room, idx) {
            for t_idx in 0..connected_tile_sets.len() {
                if connected_tile_sets[t_idx].contains(&neigh) {
                    connected_tile_sets[t_idx] =
                        HashSet::from_iter(connected_tile_sets[t_idx].clone());
                    connected_tile_sets[t_idx].insert(idx);
                    found_existing_set = true;
                }
            }
        }

        if !found_existing_set {
            let mut new_set: HashSet<usize> = HashSet::new();
            new_set.insert(idx);
            connected_tile_sets.push(new_set);
        }
    }

    merge(connected_tile_sets)
}

fn merge(tile_sets: Vec<HashSet<usize>>) -> Vec<HashSet<usize>> {
    let mut result = tile_sets.clone();

    let mut merge_again = true;
    while merge_again {
        let mut new_result = vec![];
        for set in result.iter().filter(|t| t.len() > 0) {
            new_result.push(set.clone());
        }
        result = new_result.clone();
        result = eliminate_empty(&result);
        let (r, m) = merge_iteration(&result);
        result = r;
        merge_again = m;
    }

    result
}

fn eliminate_empty(tile_sets: &Vec<HashSet<usize>>) -> Vec<HashSet<usize>> {
    tile_sets
        .iter()
        .filter(|ts| ts.len() > 0)
        .map(|ts| ts.clone())
        .collect()
}

fn merge_iteration(tile_sets: &Vec<HashSet<usize>>) -> (Vec<HashSet<usize>>, bool) {
    let mut result = tile_sets.clone();
    let mut merged = false;

    for idx1 in 0..tile_sets.len() {
        for idx2 in idx1 + 1..tile_sets.len() {
            let ts1 = result[idx1].clone();
            let ts2 = result[idx2].clone();
            if have_common_elements(&ts1, &ts2) {
                merged = true;
            } else {
                continue;
            }

            result[idx1] = union_hashsets(&ts1, &ts2);
            result[idx2] = HashSet::new();
        }
    }

    (result, merged)
}

fn neighbor_floors(room: &DungeonRoom, idx: usize) -> Vec<usize> {
    let col = room.col(idx);
    let row = room.row(idx);

    let mut result = vec![];

    if row > 0 && room.tiles[idx - (room.columns as usize)] != DungeonTile::Wall {
        result.push(idx - room.columns as usize);
    }

    if col > 0 && room.tiles[idx - 1] != DungeonTile::Wall {
        result.push(idx - 1);
    }

    if col < room.columns - 1 && room.tiles[idx + 1] != DungeonTile::Wall {
        result.push(idx + 1);
    }

    if row < room.rows - 1 && room.tiles[idx + (room.columns as usize)] != DungeonTile::Wall {
        result.push(idx + room.columns as usize);
    }

    result
}

fn union_hashsets(set1: &HashSet<usize>, set2: &HashSet<usize>) -> HashSet<usize> {
    HashSet::from_iter(set1.union(set2).map(|e| *e))
}

fn have_common_elements(set1: &HashSet<usize>, set2: &HashSet<usize>) -> bool {
    set1.intersection(set2).count() > 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn connected_tile_sets_returns_list_of_connected_areas() {
        let mut tiles = vec![DungeonTile::Floor; 16];
        for idx in vec![1, 4, 5] {
            tiles[idx] = DungeonTile::Wall;
        }
        let room = DungeonRoom {
            tiles,
            rows: 4,
            columns: 4,
            ..Default::default()
        };

        let result = connected_tile_sets(&room);

        println!("{:?}", result);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[1].len(), 12);
    }

    #[test]
    fn sorts_out_empty_hashsets() {
        let vec = vec![HashSet::new(), HashSet::from_iter(0..3)];
        let result = eliminate_empty(&vec);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
    }

    #[test]
    fn merge_iteration_returns_true_if_a_merge_occured() {
        let set1: HashSet<usize> = HashSet::from_iter(0..3);
        let set2: HashSet<usize> = HashSet::from_iter(2..5);
        let result = merge_iteration(&vec![set1, set2]);

        assert!(result.1);
        assert_eq!(result.0[0].len(), 5);
        assert_eq!(result.0[1].len(), 0);
    }

    #[test]
    fn merge_iteration_returns_false_if_no_merge_occured() {
        let set1: HashSet<usize> = HashSet::from_iter(0..3);
        let set2: HashSet<usize> = HashSet::from_iter(4..6);
        let result = merge_iteration(&vec![set1, set2]);

        assert!(!result.1);
        assert_eq!(result.0[0].len(), 3);
        assert_eq!(result.0[1].len(), 2);
    }

    #[test]
    fn union_hashset_test() {
        let mut set1: HashSet<usize> = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        set1.insert(3);
        let mut set2: HashSet<usize> = HashSet::new();
        set2.insert(1);
        set2.insert(4);

        let result = union_hashsets(&set1, &set2);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn returns_true_if_sets_have_same_entries() {
        let set1 = HashSet::from_iter(0..3);
        let set2 = HashSet::from_iter(2..4);
        let result = have_common_elements(&set1, &set2);
        assert!(result);
    }

    #[test]
    fn returns_false_if_sets_do_not_have_same_entries() {
        let set1 = HashSet::from_iter(0..2);
        let set2 = HashSet::from_iter(3..5);
        let result = have_common_elements(&set1, &set2);
        assert!(!result);
    }

    #[test]
    fn get_no_neighboring_floors() {
        let room = DungeonRoom {
            rows: 2,
            columns: 2,
            tiles: vec![
                DungeonTile::Floor,
                DungeonTile::Wall,
                DungeonTile::Wall,
                DungeonTile::Floor,
            ],
            ..Default::default()
        };

        let result = neighbor_floors(&room, 0);

        let expected: Vec<usize> = vec![];
        assert_eq!(expected, result);
    }

    #[test]
    fn identifies_neighboring_floors() {
        let room = DungeonRoom {
            rows: 3,
            columns: 3,
            tiles: vec![
                DungeonTile::Floor, //0/0
                DungeonTile::Wall,  //0/1
                DungeonTile::Wall,  //0/2
                DungeonTile::Wall,  //1/0
                DungeonTile::Exit,  //1/1
                DungeonTile::Floor, //1/2
                DungeonTile::Floor, //1/3
                DungeonTile::Floor, //2/0
                DungeonTile::Exit,  //2/1
                DungeonTile::Wall,  //2/2
            ],
            ..Default::default()
        };

        let result = neighbor_floors(&room, 4);

        let expected: Vec<usize> = vec![5, 7];
        assert_eq!(expected, result);
    }

    #[test]
    fn identifies_neighboring_floors_2() {
        let room = DungeonRoom {
            rows: 3,
            columns: 3,
            tiles: vec![
                DungeonTile::Wall,  //0/0
                DungeonTile::Floor, //0/1
                DungeonTile::Exit,  //0/2
                DungeonTile::Floor, //1/0
                DungeonTile::Floor, //1/1
                DungeonTile::Wall,  //1/2
                DungeonTile::Wall,  //1/3
                DungeonTile::Wall,  //2/0
                DungeonTile::Wall,  //2/1
                DungeonTile::Wall,  //2/2
            ],
            ..Default::default()
        };

        let result = neighbor_floors(&room, 4);

        let expected: Vec<usize> = vec![1, 3];
        assert_eq!(expected, result);
    }

    #[test]
    fn merge_tile_sets_with_intersect() {
        let set1: HashSet<usize> = HashSet::from_iter(0..3);
        let set2: HashSet<usize> = HashSet::from_iter(2..5);
        let set3: HashSet<usize> = HashSet::from_iter(6..8);
        let all = vec![set1, set2, set3];

        let result = merge(all);

        let expected_set1: HashSet<usize> = HashSet::from_iter(0..5);
        let expected_set2: HashSet<usize> = HashSet::from_iter(6..8);
        let expected_all: Vec<HashSet<usize>> = vec![expected_set1, expected_set2];
        assert_eq!(expected_all, result);
    }
}
