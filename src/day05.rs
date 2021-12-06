use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Coord {
  x: u16,
  y: u16,
}

impl FromStr for Coord {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split(',');
    Ok(Self {
      x: split.next().unwrap().parse().unwrap(),
      y: split.next().unwrap().parse().unwrap(),
    })
  }
}

#[derive(Debug, Copy, Clone)]
struct Segment {
  start: Coord,
  end: Coord,
}

impl FromStr for Segment {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split(" -> ");
    Ok(Self {
      start: split.next().unwrap().parse().unwrap(),
      end: split.next().unwrap().parse().unwrap(),
    })
  }
}

impl Segment {
  fn coords(self) -> impl Iterator<Item = Coord> {
    let start = self.start.min(self.end);
    let end = self.start.max(self.end);
    match (start, end) {
      (Coord { x: x1, y: y1 }, Coord { x: x2, y: y2 }) if x1 == x2 => {
        Box::new((y1..=y2).map(move |y| Coord { x: x1, y })) as Box<dyn Iterator<Item = Coord>>
      }
      (Coord { x: x1, y: y1 }, Coord { x: x2, y: y2 }) if y1 == y2 => {
        Box::new((x1..=x2).map(move |x| Coord { x, y: y1 }))
      }
      (Coord { x: x1, y: y1 }, Coord { x: x2, y: y2 }) if y1 < y2 => {
        Box::new((x1..=x2).zip(y1..=y2).map(|(x, y)| Coord { x, y }))
      }
      (Coord { x: x1, y: y1 }, Coord { x: x2, y: y2 }) if y1 > y2 => {
        Box::new((x1..=x2).zip((y2..=y1).rev()).map(|(x, y)| Coord { x, y }))
      }
      (l, r) => unreachable!("{:?}", (l, r)),
    }
  }
}

pub fn compute(input: &str) -> (usize, usize) {
  let segments = input
    .lines()
    .map(|l| l.parse::<Segment>().unwrap())
    .collect::<Vec<_>>();

  let straight_segments = segments
    .iter()
    .copied()
    .filter(|s| s.start.x == s.end.x || s.start.y == s.end.y)
    .collect::<Vec<_>>();

  dbg!(straight_segments.len());

  let mut straight_points = HashMap::with_capacity(4096);

  for segment in straight_segments {
    for coord in segment.coords() {
      // dbg!(Coord { x, y });
      match straight_points.entry(coord) {
        Entry::Occupied(mut o) => *o.get_mut() += 1,
        Entry::Vacant(v) => {
          v.insert(1);
        }
      }
    }
  }

  let mut points = HashMap::with_capacity(4096);

  for segment in segments {
    for coord in segment.coords() {
      match points.entry(coord) {
        Entry::Occupied(mut o) => *o.get_mut() += 1,
        Entry::Vacant(v) => {
          v.insert(1);
        }
      }
    }
  }

  (
    straight_points.values().filter(|v| **v > 1).count(),
    points.values().filter(|v| **v > 1).count(),
  )
}
