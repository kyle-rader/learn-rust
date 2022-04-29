use std::collections::HashSet;

fn count(i: usize, j: usize, map: &HashSet<(usize, usize)>) -> char {
    let mut coords: Vec<(usize, usize)> = Vec::new();
    coords.push((i, j + 1));
    coords.push((i + 1, j));
    coords.push((i + 1, j + 1));

    if i > 0 && j > 0 {
        coords.push((i - 1, j - 1));
    }
    if i > 0 {
        coords.push((i - 1, j));
        coords.push((i - 1, j + 1));
    }
    if j > 0 {
        coords.push((i, j - 1));
        coords.push((i + 1, j - 1));
    }

    let cnt: u8 = coords
        .iter()
        .filter(|cord| map.contains(*cord))
        .count()
        .try_into()
        .unwrap();

    match cnt {
        0 => ' ',
        n => (n + ('0' as u8)) as char,
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut map: HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in minefield.iter().enumerate() {
        for (j, val) in row.chars().enumerate() {
            if val == '*' {
                map.insert((i, j));
            }
        }
    }

    // lock the map
    let map = map;

    let mut result: Vec<String> = Vec::new();
    for (i, row) in minefield.iter().enumerate() {
        let mut row_marked = String::new();
        for (j, val) in row.chars().enumerate() {
            match val {
                '*' => row_marked.push(val),
                ' ' => {
                    // replace space with count of adjacent mines
                    row_marked.push(count(i, j, &map));
                }
                unexpected => panic!("unexpected input! '{unexpected}'"),
            }
        }
        result.push(row_marked);
    }
    result
}
