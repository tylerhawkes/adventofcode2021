use array_map::{ArrayMap, Indexable};
use std::collections::HashMap;

type Map = HashMap<Space, Vec<Space>>;

pub fn compute(_input: &str) -> (usize, usize) {
  use Pod::*;
  use Space::*;
  let mut space_map: Map = HashMap::new();
  space_map.insert(Resting(1), vec![Resting(2)]);
  space_map.insert(Resting(2), vec![Resting(1), Transiting(3)]);
  space_map.insert(Transiting(3), vec![Resting(2), Final(A, 1), Resting(4)]);
  space_map.insert(Final(A, 1), vec![Final(A, 2), Transiting(3)]);
  space_map.insert(Final(A, 2), vec![Final(A, 1)]);
  space_map.insert(Resting(4), vec![Transiting(3), Transiting(5)]);
  space_map.insert(Transiting(5), vec![Resting(4), Resting(6), Final(B, 1)]);
  space_map.insert(Final(B, 1), vec![Final(B, 2), Transiting(5)]);
  space_map.insert(Final(B, 2), vec![Final(B, 1)]);
  space_map.insert(Resting(6), vec![Transiting(5), Transiting(7)]);
  space_map.insert(Transiting(7), vec![Resting(6), Resting(8), Final(C, 1)]);
  space_map.insert(Final(C, 1), vec![Transiting(7), Final(C, 2)]);
  space_map.insert(Final(C, 2), vec![Final(C, 1)]);
  space_map.insert(Resting(8), vec![Transiting(7), Transiting(9)]);
  space_map.insert(Transiting(9), vec![Resting(8), Resting(10), Final(D, 1)]);
  space_map.insert(Final(D, 1), vec![Transiting(9), Final(D, 2)]);
  space_map.insert(Final(D, 2), vec![Final(D, 1)]);
  space_map.insert(Resting(10), vec![Transiting(9), Resting(11)]);
  space_map.insert(Resting(11), vec![Resting(10)]);

  let start = Cave {
    current: ArrayMap::from_closure(|p| match p {
      A => [Final(A, 1), Final(D, 2)],
      B => [Final(A, 2), Final(C, 1)],
      C => [Final(B, 2), Final(D, 1)],
      D => [Final(B, 1), Final(C, 2)],
    }),
  };

  let mut count = 0;
  let path = pathfinding::prelude::dijkstra(
    &start,
    |c| c.successors(&space_map),
    // |x| x.heuristic(&space_map),
    |x| x.heuristic(&space_map) == 0,
  )
  .unwrap();
  space_map.get_mut(&Final(A, 2)).unwrap().push(Final(A, 3));
  space_map.get_mut(&Final(B, 2)).unwrap().push(Final(B, 3));
  space_map.get_mut(&Final(C, 2)).unwrap().push(Final(C, 3));
  space_map.get_mut(&Final(D, 2)).unwrap().push(Final(D, 3));
  space_map.insert(Final(A, 3), vec![Final(A, 2), Final(A, 4)]);
  space_map.insert(Final(B, 3), vec![Final(B, 2), Final(B, 4)]);
  space_map.insert(Final(C, 3), vec![Final(C, 2), Final(C, 4)]);
  space_map.insert(Final(D, 3), vec![Final(D, 2), Final(D, 4)]);
  space_map.insert(Final(A, 4), vec![Final(A, 3)]);
  space_map.insert(Final(B, 4), vec![Final(B, 3)]);
  space_map.insert(Final(C, 4), vec![Final(C, 3)]);
  space_map.insert(Final(D, 4), vec![Final(D, 3)]);

  let start = Cave {
    current: ArrayMap::from_closure(|p| match p {
      A => [Final(A, 1), Final(C, 3), Final(D, 2), Final(D, 4)],
      B => [Final(A, 4), Final(B, 3), Final(C, 1), Final(C, 2)],
      C => [Final(B, 2), Final(B, 4), Final(D, 1), Final(D, 3)],
      D => [Final(A, 2), Final(A, 3), Final(B, 1), Final(C, 4)],
    }),
  };
  let path2 = pathfinding::prelude::dijkstra(
    &start,
    |c| c.successors(&space_map),
    // |x| x.heuristic(&space_map),
    |x| x.heuristic(&space_map) == 0,
  )
  .unwrap();
  // dbg!(path2);
  (path.1, path2.1)
}

#[derive(Copy, Clone, Debug, array_map::Indexable, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Pod {
  A,
  B,
  C,
  D,
}

impl Pod {
  fn step_cost(self) -> usize {
    match self {
      Pod::A => 1,
      Pod::B => 10,
      Pod::C => 100,
      Pod::D => 1000,
    }
  }
  fn possible_moves<const N: usize>(
    self,
    current: Space,
    final_clear: bool,
  ) -> impl Iterator<Item = Space> {
    use Pod::*;
    use Space::*;
    let f = match self {
      A => [Final(A, 1), Final(A, 2), Final(A, 3), Final(A, 4)],
      B => [Final(B, 1), Final(B, 2), Final(B, 3), Final(B, 4)],
      C => [Final(C, 1), Final(C, 2), Final(C, 3), Final(C, 4)],
      D => [Final(D, 1), Final(D, 2), Final(D, 3), Final(D, 4)],
    };
    [
      Resting(1),
      Resting(2),
      Resting(4),
      Resting(6),
      Resting(8),
      Resting(10),
      Resting(11),
    ]
    .into_iter()
    // TODO: figure out which final they can go into since they need to stack up from the bottom
    .chain(f.into_iter().take(N).rev())
    .filter(move |s| {
      if current.is_resting() {
        return !s.is_resting() && final_clear;
      }
      if current.is_final() {
        return s.is_resting() || final_clear;
      }
      unreachable!()
    })
  }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Space {
  Resting(u8),
  Transiting(u8),
  Final(Pod, u8),
}

impl Space {
  fn is_resting(self) -> bool {
    matches!(self, Self::Resting(_))
  }
  fn is_final(self) -> bool {
    matches!(self, Self::Final(_, _))
  }
  fn visit(self) -> u32 {
    match self {
      Self::Resting(u) | Self::Transiting(u) => 1 << u,
      Self::Final(p, i) => 1 << (12 + (p.index() as u32 * 4 + i as u32)),
    }
  }
  fn distance(self, end: Self, map: &Map, taken: u8, visited: u32) -> Option<u8> {
    if self == end {
      return Some(taken);
    }
    let visited = visited | self.visit();
    for &next in map.get(&self).unwrap().iter() {
      if next.visit() & visited == 0 {
        if let Some(x) = next.distance(end, map, taken + 1, visited) {
          return Some(x);
        }
      }
    }
    None
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Cave<const N: usize> {
  current: ArrayMap<Pod, [Space; N], { Pod::SIZE }>,
}

impl<const N: usize> Cave<N> {
  fn heuristic(&self, map: &Map) -> usize {
    self
      .current
      .iter()
      .map(|(p, s)| {
        if s.iter().all(|s| match s {
          Space::Final(p1, _) => *p1 == p,
          _ => false,
        }) {
          0
        } else {
          // TODO: Figure out how to find a real minimal cost to get to the end
          s.iter()
            .map(|s| (s.distance(Space::Final(p, 2), map, 0, 0).unwrap() as usize) * p.step_cost())
            .sum::<usize>()
        }
      })
      .sum()
  }
  fn successors(self, map: &Map) -> impl Iterator<Item = (Self, usize)> {
    self
      .pods()
      .filter(|(p, s, _)| {
        !(self.final_clear(*p)
          && match s {
            Space::Final(p1, _) => p == p1,
            _ => false,
          })
      })
      .flat_map(move |(p, s, index)| {
        p.possible_moves::<N>(s, self.final_clear(p))
          .map(move |e| (p, s, e, index))
          .filter_map(move |(p, start, end, index)| {
            self.can_get_to(start, end, map, 0, 0).map(|steps| {
              let mut new = self.clone();
              new.current[p][index as usize] = end;
              (new, steps as usize * p.step_cost())
            })
          })
      })
      .collect::<Vec<_>>()
      .into_iter()
  }
  fn can_move(&self, space: Space) -> bool {
    !self.spaces().any(|x| x == space)
  }
  fn can_get_to(&self, start: Space, end: Space, map: &Map, taken: u8, visited: u32) -> Option<u8> {
    if start == end {
      return Some(taken);
    }
    let visited = visited | start.visit();
    for &next in map.get(&start).unwrap().iter() {
      if next.visit() & visited == 0 && self.can_move(next) {
        if let Some(x) = self.can_get_to(next, end, map, taken + 1, visited) {
          return Some(x);
        }
      }
    }
    None
  }
  fn final_clear(&self, p1: Pod) -> bool {
    self.pods().all(|(p2, s, _)| match s {
      Space::Final(p3, _) if p3 == p1 => p1 == p2,
      _ => true,
    })
  }
  fn spaces(&self) -> impl Iterator<Item = Space> + '_ {
    self.current.values().flat_map(|x| x.iter().copied())
  }
  fn pods(&self) -> impl Iterator<Item = (Pod, Space, u8)> + '_ {
    self.current.iter().flat_map(|(p, s)| {
      s.iter()
        .copied()
        .enumerate()
        .map(move |(index, s)| (p, s, index as u8))
    })
  }
}
