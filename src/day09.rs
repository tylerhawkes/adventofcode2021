use std::collections::{HashSet, VecDeque};

pub fn compute(input: &str) -> (usize, usize) {
  let heights = input
    .lines()
    .map(|l| l.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let mut risk = 0;
  let mut min_points = Vec::with_capacity(500);
  for (y, row) in heights.iter().enumerate() {
    for (x, val) in row.iter().copied().enumerate() {
      if heights[y][x] == 9 {
        continue;
      }
      if neighbor_values(x, y, &heights).all(|n| n > val) {
        risk += val as usize + 1;
        min_points.push((x, y, 0));
      }
    }
  }
  for (x, y, count) in &mut min_points {
    *count = explore_basin(*x, *y, &heights);
  }
  min_points.sort_unstable_by_key(|x| x.2);
  let product = min_points.iter().rev().take(3).map(|x| x.2).product();
  (risk, product)
}

fn neighbor_values<'a>(
  x: usize,
  y: usize,
  heights: &'a [Vec<u8>],
) -> impl Iterator<Item = u8> + 'a {
  neighbors(x, y, heights).map(|(x, y)| heights[y][x])
}

fn neighbors<'a>(
  x: usize,
  y: usize,
  heights: &'a [Vec<u8>],
) -> impl Iterator<Item = (usize, usize)> + 'a {
  let max_x = heights[0].len();
  let max_y = heights.len();
  [[-1, 0], [0, -1], [1, 0], [0, 1]]
    .into_iter()
    .filter_map(move |[nx, ny]| {
      let neighbor_x: isize = x as isize + nx;
      let neighbor_y: isize = y as isize + ny;
      ((neighbor_x as usize) < max_x && (neighbor_y as usize) < max_y)
        .then(|| (neighbor_x as usize, neighbor_y as usize))
    })
}

fn explore_basin(x: usize, y: usize, heights: &[Vec<u8>]) -> usize {
  let mut visited = HashSet::new();
  let mut to_visit = neighbors(x, y, &heights).collect::<VecDeque<_>>();
  let mut count = 1;
  visited.insert((x, y));
  while !to_visit.is_empty() {
    let (x, y) = to_visit.pop_front().unwrap();
    if heights[y][x] == 9 || visited.contains(&(x, y)) {
      continue;
    }
    count += 1;
    visited.insert((x, y));
    for n in neighbors(x, y, heights) {
      if !visited.contains(&n) {
        to_visit.push_back(n);
      }
    }
  }
  count
}
