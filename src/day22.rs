use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn compute(input: &str) -> (usize, usize) {
  let cuboids = input
    .lines()
    .map(|l| l.parse::<Cuboid>().unwrap())
    .collect::<Vec<_>>();
  let mut on_cubes = HashSet::<(i8, i8, i8)>::with_capacity(2 << 18);
  cuboids.iter().take(10).for_each(|c| {
    for (x, y, z) in c.points().map(|(x, y, z)| (x as i8, y as i8, z as i8)) {
      if c.on {
        on_cubes.insert((x, y, z));
      } else {
        on_cubes.remove(&(x, y, z));
      }
    }
  });
  let splits = (0..3)
    .map(|i| {
      let mut splits = cuboids
        .iter()
        .flat_map(|x| [*x.xyz[i].start(), *x.xyz[i].end() + 1])
        .collect::<Vec<_>>();
      splits.sort_unstable();
      splits.dedup();
      splits
    })
    .collect::<Vec<_>>();
  let mut regions = HashSet::with_capacity(1 << 28);
  for (i, cuboid) in cuboids.iter().enumerate() {
    println!(
      "Handling cuboid {}, {:?}. Regions len: {}",
      i,
      cuboid,
      regions.len(),
    );
    let indices = splits
      .iter()
      .zip(cuboid.xyz.iter())
      .map(|(s, r)| {
        s.binary_search(r.start()).unwrap() as u16..s.binary_search(&(r.end() + 1)).unwrap() as u16
      })
      .collect::<Vec<_>>();
    println!("Indices: {:?}", indices);
    for x in indices[0].clone() {
      for y in indices[1].clone() {
        for z in indices[2].clone() {
          if cuboid.on {
            regions.insert((x, y, z));
          } else {
            regions.remove(&(x, y, z));
          }
        }
      }
    }
  }
  dbg!(regions.len());
  let total_set = regions
    .into_iter()
    .map(|(x, y, z)| {
      let x_range = (splits[0][x as usize + 1] - splits[0][x as usize]) as usize;
      let y_range = (splits[1][y as usize + 1] - splits[1][y as usize]) as usize;
      let z_range = (splits[2][z as usize + 1] - splits[2][z as usize]) as usize;
      x_range * y_range * z_range
    })
    .sum::<usize>();
  // dbg!(Cuboid::process(&cuboids[..20.min(cuboids.len())]));
  println!("Correct: 2758514936282235");
  (on_cubes.len(), total_set)
}

#[derive(Debug, Clone)]
struct Cuboid {
  on: bool,
  xyz: [RangeInclusive<isize>; 3],
}

impl Cuboid {
  fn process(cuboids: &[Self]) -> usize {
    let mut on_cuboids: Vec<Cuboid> = vec![];
    let mut processing_cuboids: Vec<Cuboid> = vec![];
    for cuboid in cuboids {
      println!(
        "processing cuboid: {:?}, total: {}",
        cuboid,
        on_cuboids.len()
      );
      processing_cuboids.clear();
      let mut processed = false;
      for on in &on_cuboids {
        if let Some(splits) = on.split_overlap(cuboid) {
          processed = true;
          // println!("Extending splits");
          processing_cuboids.extend(splits);
        } else {
          // println!("Extending original");
          processing_cuboids.push(on.clone());
        }
      }
      if !processed {
        processing_cuboids.push(cuboid.clone());
      }
      std::mem::swap(&mut on_cuboids, &mut processing_cuboids);
    }
    on_cuboids.iter().map(Self::count).sum()
  }
  fn count(&self) -> usize {
    self
      .xyz
      .iter()
      .map(|r| (*r.end() - *r.start()) as usize + 1)
      .product()
  }
  fn split_overlap<'a>(&'a self, new: &'a Self) -> Option<impl Iterator<Item = Self> + 'a> {
    assert!(self.on);
    let x_overlap = overlap(&self.xyz[0], &new.xyz[0]);
    let y_overlap = overlap(&self.xyz[1], &new.xyz[1]);
    let z_overlap = overlap(&self.xyz[2], &new.xyz[2]);
    match (x_overlap, y_overlap, z_overlap) {
      (Some(x_overlap), Some(y_overlap), Some(z_overlap)) => Some(
        x_overlap
          .flat_map(move |x| y_overlap.clone().map(move |y| (x.clone(), y)))
          .flat_map(move |(x, y)| z_overlap.clone().map(move |z| [x.clone(), y.clone(), z]))
          .filter_map(move |xyz| {
            let c = Cuboid { xyz, on: true };
            let start_point = c.start_point();
            if !new.on && new.contains(start_point) {
              // println!("filtering: {:?}", c.xyz);
              return None;
            }
            if self.contains(start_point) || (new.on && new.contains(start_point)) {
              // println!("keeping  : {:?}", c.xyz);
              return Some(c);
            }
            // println!("dropping : {:?}", c.xyz);
            None
          }),
      ),
      _ => None,
    }
  }
  fn contains(&self, (x, y, z): (isize, isize, isize)) -> bool {
    self
      .xyz
      .iter()
      .zip([x, y, z].iter())
      .all(|(r, p)| r.contains(p))
  }
  fn points(&self) -> impl Iterator<Item = (isize, isize, isize)> + '_ {
    self.xyz[0]
      .clone()
      .flat_map(|x| self.xyz[1].clone().map(move |y| (x, y)))
      .flat_map(|(x, y)| self.xyz[2].clone().map(move |z| (x, y, z)))
  }
  fn start_point(&self) -> (isize, isize, isize) {
    (
      *self.xyz[0].start(),
      *self.xyz[1].start(),
      *self.xyz[2].start(),
    )
  }
}

fn overlap(
  old: &RangeInclusive<isize>,
  new: &RangeInclusive<isize>,
) -> Option<impl Iterator<Item = RangeInclusive<isize>> + Clone> {
  if new.end() < old.start() || old.end() < new.start() {
    return None;
  }
  let ranges = if old.contains(new.start()) && old.contains(new.end()) {
    [
      *old.start()..=*new.start() - 1,
      new.clone(),
      *new.end() + 1..=*old.end(),
    ]
  } else if new.contains(old.start()) && new.contains(old.end()) {
    [
      *new.start()..=*old.start() - 1,
      old.clone(),
      *old.end() + 1..=*new.end(),
    ]
  } else if new.contains(old.start()) {
    [
      *new.start()..=*old.start() - 1,
      *old.start()..=*new.end(),
      *new.end() + 1..=*old.end(),
    ]
  } else if new.contains(old.end()) {
    [
      *old.start()..=*new.start() - 1,
      *new.start()..=*old.end(),
      *old.end() + 1..=*new.end(),
    ]
  } else {
    unreachable!()
  };

  Some(ranges.into_iter().filter(|r| !r.is_empty()))
}

impl FromStr for Cuboid {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let x = s.split(' ').collect::<Vec<_>>();
    let on = match x[0] {
      "on" => true,
      "off" => false,
      _ => unreachable!(),
    };
    let xyz = x[1]
      .split(',')
      .map(|xyz| xyz.split('=').skip(1).next().unwrap())
      .map(|r| {
        let x = r
          .split("..")
          .map(|i| i.parse::<isize>().unwrap())
          .collect::<Vec<_>>();
        x[0]..=x[1]
      })
      .collect::<Vec<_>>();
    Ok(Self {
      on,
      xyz: xyz.try_into().unwrap(),
    })
  }
}
