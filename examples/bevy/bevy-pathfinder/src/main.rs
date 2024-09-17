use anyhow::*;
use pathfinding::prelude::*;

#[derive(Clone, Default, Debug)]
pub struct PathCost {
    pub path: Vec<(usize, usize)>,
    pub cost: usize,
}

fn successors(
    walkables: &Vec<Vec<bool>>,
    &(x, y): &(usize, usize),
) -> Vec<((usize, usize), usize)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter_map(|(nx, ny)| walkables[ny][nx].then_some(((nx, ny), 1)))
        .collect()
}

fn distance(&(x1, y1): &(usize, usize), &(x2, y2): &(usize, usize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

pub fn find_path(
    walkables: &Vec<Vec<bool>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Result<PathCost> {
    let mut counter = 0;
    let (path, cost) = astar(
        &start,
        |n| {
            counter += 1;
            successors(&walkables, n)
        },
        |n| distance(n, &goal),
        |n| n == &goal,
    )
    .expect("path not found");
    assert_eq!(cost, 8);
    assert!(path.iter().all(|&(nx, ny)| walkables[ny][nx]));
    assert_eq!(counter, 11);

    Ok(PathCost { path, cost })
}

fn main() {
    let map: &str = "\
#########
#.#.....#
###.##..#
#...#...#
#...#...#
#...#...#
#...#...#
#########
";

    let walkables: Vec<Vec<bool>> = map
        .lines()
        .map(|l| l.chars().map(|c| c == '.').collect())
        .collect();

    let start: (usize, usize) = (2, 3);
    let goal: (usize, usize) = (6, 3);

    let result = find_path(&walkables, start, goal);
    println!("{:?}", result);
}
