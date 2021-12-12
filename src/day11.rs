use std::collections::HashSet;

pub fn compute(input: &str) -> (usize, usize) {
  let mut grid = input
    .lines()
    .map(|l| l.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let flashes = (0..100).map(|_| step(&mut grid)).sum();
  let mut i = 100;
  loop {
    i += 1;
    step(&mut grid);
    if grid.iter().all(|row| row.iter().all(|u| *u == 0)) {
      break;
    }
  }
  (flashes, i)
}

fn step(grid: &mut [Vec<u8>]) -> usize {
  let mut flashed = HashSet::new();
  grid
    .iter_mut()
    .for_each(|r| r.iter_mut().for_each(|u| *u += 1));
  let mut c = true;
  while c {
    c = false;
    for y in 0..grid.len() {
      for x in 0..grid[0].len() {
        if grid[y][x] > 9 && !flashed.contains(&(x, y)) {
          flashed.insert((x, y));
          grid[y][x] = 0;
          c = true;
          for (nx, ny) in neighbors(x, y, grid[0].len(), grid.len()) {
            grid[ny][nx] += 1;
          }
        }
      }
    }
  }
  for (x, y) in &flashed {
    grid[*y][*x] = 0;
  }
  // for row in grid.iter() {
  //   row.iter().for_each(|u| print!("{}", u.min(&9)));
  //   println!();
  // }
  // println!("Flashes: {}\n", flashed.len());
  flashed.len()
}

fn neighbors(
  x: usize,
  y: usize,
  x_limit: usize,
  y_limit: usize,
) -> impl Iterator<Item = (usize, usize)> {
  [
    (-1_isize, -1_isize),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
  ]
  .into_iter()
  .filter_map(move |(x_offset, y_offset)| {
    let x = (x as isize + x_offset) as usize;
    let y = (y as isize + y_offset) as usize;
    (x < x_limit && y < y_limit).then(|| (x, y))
  })
}
