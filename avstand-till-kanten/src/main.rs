use std::{cmp::min, io::stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let input = buf
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let (r, k) = (input[0], input[1]);
    let mut rectangle: Vec<Vec<usize>> = vec![vec![0; k]; r];

    #[allow(clippy::needless_range_loop)]
    for y in 0..r {
        for x in 0..k {
            let distance = manhattan_to_edge(r, k, x, y);
            let jumps = distance + 1;
            rectangle[y][x] = jumps
        }
    }
}

// https://en.wikipedia.org/wiki/Taxicab_geometry
// we can calculate the so-called Manhattan Distance
// of the target to the each edge of the rectangle with:
//                    ---------------
//                   | ↑: y          |
//                   | ←: x          |
//                   | →: k - 1 - x  |
//                   | ↓: r - 1 - y  |
//                    ---------------
// the minimum of these distances will be the answer
fn manhattan_to_edge(r: usize, k: usize, x: usize, y: usize) -> usize {
    let top = y;
    let left = x;
    let right = k - 1 - x;
    let bottom = r - 1 - x;

    min(min(top, bottom), min(left, right))
}
