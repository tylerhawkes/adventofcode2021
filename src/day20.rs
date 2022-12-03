use std::collections::{HashMap, HashSet};
// 5622, 20395
pub fn compute(input: &str) -> (usize, usize) {
  let x = input.split("\n\n").collect::<Vec<_>>();
  let image_enhancement = x[0].chars().map(|c| c == '#').collect::<Vec<_>>();
  let image = x[1]
    .lines()
    .enumerate()
    .flat_map(|(y, l)| {
      l.chars()
        .enumerate()
        .map(move |(x, c)| ((x as i16, y as i16), c == '#'))
    })
    .collect::<HashMap<_, _>>();
  let image = Image { set_bits: image };
  image.print();
  let image_1 = image.enhance(&image_enhancement, 1);
  image_1.print();
  let image_2 = image_1.enhance(&image_enhancement, 2);
  image_2.print();
  (image_2.set_bits.values().filter(|x| **x).count(), 0)
}

struct Image {
  set_bits: HashMap<(i16, i16), bool>,
}

impl Image {
  fn extended_bound_iter(&self) -> impl Iterator<Item = (i16, i16)> + '_ {
    let (min_x, min_y, max_x, max_y) = self
      .set_bits
      .keys()
      .fold((i16::MAX, i16::MAX, i16::MIN, i16::MIN), |l, &r| {
        (l.0.min(r.0), l.1.min(r.1), l.2.max(r.0), l.3.max(r.1))
      });
    ((min_y - 1)..=(max_y + 1)).flat_map(move |y| ((min_x - 1)..=(max_x + 1)).map(move |x| (x, y)))
  }
  fn print(&self) {
    let mut last_y = i16::MIN;
    for (x, y) in self.extended_bound_iter() {
      if y != last_y {
        last_y = y;
        println!();
      }
      if self.set_bits.get(&(x, y)).copied().unwrap_or(false) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!();
  }
  fn enhance(&self, enhancement: &[bool], iter: i16) -> Self {
    let mut new = HashMap::with_capacity(self.set_bits.len() * 2);
    for (x, y) in self.extended_bound_iter() {
      new.insert((x, y), lookup(&self.set_bits, x, y, enhancement, iter));
    }
    Self { set_bits: new }
  }
}

fn lookup(image: &HashMap<(i16, i16), bool>, x: i16, y: i16, enhance: &[bool], iter: i16) -> bool {
  let mut r = 0;
  let mut count = 8;
  for y in (y - 1)..=(y + 1) {
    for x in (x - 1)..=(x + 1) {
      r |= (image
        .get(&(x, y))
        .copied()
        .unwrap_or(iter.trailing_zeros() == 0) as usize)
        << count;
      count -= 1;
    }
  }
  let v = enhance[r];
  // println!(" = {:09b}, {:03}, {}", r, r, if v { "#" } else { "." });
  v
}
