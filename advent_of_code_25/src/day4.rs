use crate::utils::Map;

pub fn part1() {
    let map = Map::from_file("data/day4.txt").unwrap();
    let mut total = 0;
    for pos in map.positions() {
        let c = map.get_pos(pos).unwrap();
        if c == '@' && map.count_free_neighbors8(pos) >= 5 {
            total += 1;
        }
    }
    println!("Number of free rolls: {total}");
}

pub fn part2() {
    let mut map = Map::from_file("data/day4.txt").unwrap();
    let mut total = 0;
    loop {
        let mut to_remove = Vec::new();
        for pos in map.positions() {
            let c = map.get_pos(pos).unwrap();
            if c == '@' && map.count_free_neighbors8(pos) >= 5 {
                to_remove.push(pos);
            }
        }
        if to_remove.is_empty() {
            break;
        }

        for pos in to_remove {
            map.set_pos(pos, '.');
            total += 1;
        }
    }
    println!("Number of removed rolls: {total}");
}
