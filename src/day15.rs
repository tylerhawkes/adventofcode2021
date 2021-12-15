use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

pub fn compute(input: &str) -> (usize, usize) {
  let risk_map = input
    .lines()
    .map(|l| l.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let risk = search(&risk_map);
  let big_risk_map = expand_map(&risk_map);
  let risk_5 = search(&big_risk_map);
  let max_x = big_risk_map[0].len();
  let max_y = big_risk_map.len();
  let start = Instant::now();
  let (_path, astar_risk) = pathfinding::prelude::astar(
    &(0_usize, 0_usize),
    |&(x, y)| neighbors((x, y), max_x, max_y).map(|(x, y)| ((x, y), big_risk_map[y][x] as usize)),
    |&(x, y)| max_x - 1 - x + max_y - 1 - y,
    |&(x, y)| x == max_x - 1 && y == max_y - 1,
  )
  .unwrap();
  dbg!((astar_risk, start.elapsed()));
  (risk, risk_5)
}

fn neighbors<'a>(
  (x, y): (usize, usize),
  max_x: usize,
  max_y: usize,
) -> impl Iterator<Item = (usize, usize)> + 'a {
  [[-1, 0], [0, -1], [1, 0], [0, 1]]
    .into_iter()
    .filter_map(move |[nx, ny]| {
      let neighbor_x: usize = (x as isize + nx) as usize;
      let neighbor_y: usize = (y as isize + ny) as usize;
      (neighbor_x < max_x && neighbor_y < max_y).then(|| (neighbor_x, neighbor_y))
    })
}

fn search(map: &[Vec<u8>]) -> usize {
  let start = Instant::now();
  // set r to an unoptimal solution going along the top and down the right side
  let mut r = map[0].iter().skip(1).map(|x| *x as usize).sum::<usize>()
    + map
      .iter()
      .skip(1)
      .map(|row| row[row.len() - 1] as usize)
      .sum::<usize>();
  let mut heap = BinaryHeap::with_capacity(4096);
  let mut visited = HashMap::with_capacity(4096);
  heap.push((Reverse(0), (0, 0), 0));
  let mut count = 0;
  let mut max_heap = 0;
  while !heap.is_empty() {
    count += 1;
    max_heap = max_heap.max(heap.len());
    let (_cost, (x, y), risk) = heap.pop().unwrap();
    if risk > r {
      // No need to continue searching a branch that is greater than our least costly solution
      continue;
    }
    // println!("{:05} {:?}", count, ((x, y), risk));
    for (x, y) in neighbors((x, y), map[0].len(), map.len()) {
      let value = map[y][x] as usize;
      let risk = value + risk;
      if x == map[0].len() - 1 && y == map.len() - 1 {
        if risk < r {
          println!(
            "new risk: {}, took {:?} on iter {}",
            risk,
            start.elapsed(),
            count
          );
          r = risk;
        }
        continue;
      }
      if risk < *visited.entry((x as u16, y as u16)).or_insert(risk + 1) {
        *visited.entry((x as u16, y as u16)).or_insert(risk) = risk;
        let v = (Reverse(risk), (x, y), risk);
        heap.push(v);
      }
    }
  }
  println!(
    "search took {:?} with {} iterations. Max heap size = {}",
    start.elapsed(),
    count,
    max_heap,
  );
  r
}

fn expand_map(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let mut big_map = map.clone();

  big_map
    .iter_mut()
    .zip(map.iter())
    .for_each(|(big_row, orig_row)| {
      for i in 1..5 {
        big_row.extend(orig_row.iter().copied().map(|x| x + i));
      }
    });

  let big_map_orig = big_map.clone();

  for i in 1..5 {
    for row in &big_map_orig {
      big_map.push(row.iter().copied().map(|x| x + i).collect::<Vec<_>>());
    }
  }

  big_map.iter_mut().for_each(|row| {
    row.iter_mut().for_each(|r| {
      if *r > 9 {
        *r -= 9;
      }
    });
  });

  big_map
}
