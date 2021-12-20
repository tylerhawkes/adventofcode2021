use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

pub fn compute(input: &str) -> (usize, usize) {
  let scanners = input.parse::<Scanners>().unwrap();
  let rotated = scanners.rotate_all();
  let mut hash_set = HashSet::new();
  for r in &rotated.scanners {
    hash_set.extend(r.beacons.iter().copied());
  }
  (hash_set.len(), rotated.max_manhattan_distance())
}

type Vector = (i16, i16, i16);

struct Scanners {
  scanners: Vec<Scanner>,
}

impl FromStr for Scanners {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let scanners = s
      .split("\n\n")
      .map(|b| b.parse::<Scanner>().unwrap())
      .collect::<Vec<_>>();
    Ok(Self { scanners })
  }
}

impl Scanners {
  fn max_manhattan_distance(&self) -> usize {
    let mut max_dist = 0;
    for l in self.scanners.iter() {
      let lb = Beacon::t(l.coords);
      for r in self.scanners.iter() {
        let rb = Beacon::t(r.coords);
        let (x, y, z) = lb.abs_dist(rb);
        let manhattan_distance = x.abs() as usize + y.abs() as usize + z.abs() as usize;
        if manhattan_distance > max_dist {
          max_dist = manhattan_distance
        }
      }
    }
    max_dist
  }
  fn rotate_all(&self) -> Self {
    let mut scanners = Vec::with_capacity(self.scanners.len());
    let mut scanned = vec![false; self.scanners.len()];
    scanners.push(self.scanners[0].clone());
    scanned[0] = true;
    while !scanned.iter().all(|x| *x) {
      for (i, scanner) in self.scanners.iter().enumerate() {
        if scanned[i] {
          continue;
        }
        for final_scanner in &scanners {
          if let Some((rotation, vector)) = final_scanner.overlaps(scanner) {
            scanners.push(scanner.adjust_to_other_space(rotation, vector));
            scanned[i] = true;
            break;
          }
        }
      }
    }
    Self { scanners }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scanner {
  beacons: Vec<Beacon>,
  coords: Vector,
  abs_dist: HashMap<Beacon, HashSet<Vector>>,
}

impl FromStr for Scanner {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let beacons = s
      .lines()
      .skip(1)
      .map(|l| l.parse::<Beacon>().unwrap())
      .collect::<Vec<_>>();
    Ok(Self::new(beacons).with_abs_dist())
  }
}

impl Scanner {
  fn with_abs_dist(mut self) -> Self {
    self.abs_dist = self
      .beacons
      .iter()
      .copied()
      .map(|b| {
        (
          b,
          self
            .beacons
            .iter()
            .copied()
            .map(|b2| b.abs_dist(b2))
            .filter(|d| *d != (0, 0, 0))
            .collect(),
        )
      })
      .collect();
    self
  }
  fn with_coords(mut self, coords: Vector) -> Self {
    self.coords = coords;
    self
  }
  fn new(beacons: Vec<Beacon>) -> Self {
    Self {
      beacons,
      coords: (0, 0, 0),
      abs_dist: HashMap::new(),
    }
    .with_abs_dist()
  }
  fn adjust_to_other_space(&self, rotation: Rotation, offset: Vector) -> Self {
    let beacons = self
      .beacons
      .iter()
      .copied()
      .map(|r| r.rotate(rotation).offset(offset))
      .collect::<Vec<_>>();
    Self::new(beacons).with_coords(offset)
  }
  fn overlaps(&self, other: &Self) -> Option<(Rotation, Vector)> {
    for &beacon in self.beacons.iter() {
      let self_distances = self.abs_dist.get(&beacon).unwrap();
      for &other_beacon in &other.beacons {
        let other_distances = other.abs_dist.get(&other_beacon).unwrap();
        let overlap = self_distances
          .intersection(&other_distances)
          .collect::<Vec<_>>();
        if overlap.len() >= 11 {
          let self_dists = self
            .beacons
            .iter()
            .copied()
            .map(|x| x.dist(beacon))
            .filter(|x| *x != (0, 0, 0))
            .collect::<HashSet<_>>();
          for rotation in Rotation::rotations() {
            let other_dists = other
              .beacons
              .iter()
              .copied()
              .map(|x| rotation.apply(x.dist(other_beacon)))
              .filter(|x| *x != (0, 0, 0))
              .collect::<HashSet<_>>();
            let intersection = self_dists.intersection(&other_dists).collect::<Vec<_>>();
            if intersection.len() >= 11 {
              return Some((rotation, beacon.dist(other_beacon.rotate(rotation))));
            }
          }
        }
      }
    }
    None
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Beacon {
  x: i16,
  y: i16,
  z: i16,
}

impl FromStr for Beacon {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let v = s
      .split(',')
      .map(|x| x.parse::<i16>().unwrap())
      .collect::<Vec<_>>();
    Ok(Self {
      x: v[0],
      y: v[1],
      z: v[2],
    })
  }
}

impl From<Beacon> for Vector {
  fn from(Beacon { x, y, z }: Beacon) -> Self {
    (x, y, z)
  }
}

impl Beacon {
  fn n(x: i16, y: i16, z: i16) -> Self {
    Self { x, y, z }
  }
  fn t((x, y, z): Vector) -> Self {
    Self::n(x, y, z)
  }
  fn rotate(self, rotation: Rotation) -> Self {
    Self::t(rotation.apply(self))
  }
  fn offset(self, (x, y, z): Vector) -> Self {
    Self {
      x: self.x + x,
      y: self.y + y,
      z: self.z + z,
    }
  }
  fn abs_dist(self, other: Self) -> Vector {
    let dx = (self.x - other.x).abs();
    let dy = (self.y - other.y).abs();
    let dz = (self.z - other.z).abs();
    let min = dx.abs().min(dy.abs()).min(dz.abs()) as i16;
    let max = dx.abs().max(dy.abs()).max(dz.abs()) as i16;
    let mid = dx + dy + dz - min - max;
    (min, mid, max)
  }
  fn dist(self, other: Self) -> Vector {
    (self.x - other.x, self.y - other.y, self.z - other.z)
  }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct OrderedFloat(f32);

impl Eq for OrderedFloat {}
impl Ord for OrderedFloat {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.partial_cmp(&other.0).unwrap()
  }
}
impl Hash for OrderedFloat {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.to_bits().hash(state)
  }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Coord {
  X,
  Y,
  Z,
}

impl Coord {
  fn apply<I: Into<Vector>>(self, b: I) -> i16 {
    let b = b.into();
    match self {
      Coord::X => b.0,
      Coord::Y => b.1,
      Coord::Z => b.2,
    }
  }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Sign {
  Neg(Coord),
  Pos(Coord),
}

impl Sign {
  fn apply<I: Into<Vector>>(self, b: I) -> i16 {
    let b = b.into();
    match self {
      Sign::Neg(c) => -c.apply(b),
      Sign::Pos(c) => c.apply(b),
    }
  }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Rotation(Sign, Sign, Sign);

impl Rotation {
  fn rotations() -> impl Iterator<Item = Self> {
    let o = Rotation;
    let px = Sign::Pos(Coord::X);
    let py = Sign::Pos(Coord::Y);
    let pz = Sign::Pos(Coord::Z);
    let nx = Sign::Neg(Coord::X);
    let ny = Sign::Neg(Coord::Y);
    let nz = Sign::Neg(Coord::Z);
    [
      o(px, py, pz),
      o(nx, py, pz),
      o(px, ny, pz),
      o(px, py, nz),
      o(nx, ny, pz),
      o(nx, py, nz),
      o(px, ny, nz),
      o(nx, ny, nz),
    ]
    .into_iter()
    .flat_map(move |order| {
      let Rotation(x, y, z) = order;
      [
        o(x, y, z),
        o(x, z, y),
        o(y, x, z),
        o(y, z, x),
        o(z, x, y),
        o(z, y, x),
      ]
      .into_iter()
    })
  }
  fn apply<I: Into<Vector>>(self, b: I) -> Vector {
    let b = b.into();
    (self.0.apply(b), self.1.apply(b), self.2.apply(b))
  }
}
