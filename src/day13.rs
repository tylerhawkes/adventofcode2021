use std::collections::HashSet;

pub fn compute(input: &str) -> (usize, usize) {
  let mut split = input.split("\n\n");
  let dots = split.next().unwrap();
  let mut dots = dots
    .lines()
    .map(|l| {
      let mut l_split = l.split(',');
      (
        l_split.next().unwrap().parse::<usize>().unwrap(),
        l_split.next().unwrap().parse::<usize>().unwrap(),
      )
    })
    .collect::<HashSet<_>>();
  let instructions = split.next().unwrap();
  let instructions = instructions
    .lines()
    .map(|l| {
      let mut l_split = l.strip_prefix("fold along ").unwrap().split('=');
      match l_split.next().unwrap() {
        "x" => (l_split.next().unwrap().parse::<usize>().unwrap(), 0),
        "y" => (0, l_split.next().unwrap().parse::<usize>().unwrap()),
        _ => unreachable!(),
      }
    })
    .collect::<Vec<_>>();

  dbg!(dots.len());
  let mut first_left = 0;
  for (x, y) in instructions {
    let changes = match (x, y) {
      (x, 0) => dots
        .iter()
        .copied()
        .filter_map(|d| (d.0 > x).then(|| (d, (x - (d.0 - x), d.1))))
        .collect::<Vec<_>>(),
      (0, y) => dots
        .iter()
        .copied()
        .filter_map(|d| (d.1 > y).then(|| (d, (d.0, y - (d.1 - y)))))
        .collect::<Vec<_>>(),
      _ => unreachable!(),
    };
    for (orig, new) in changes {
      dots.remove(&orig);
      dots.insert(new);
    }
    if first_left == 0 {
      first_left = dots.len();
    }
  }
  let (x_max, y_max) = dots
    .iter()
    .copied()
    .fold((0, 0), |next, max| (max.0.max(next.0), max.1.max(next.1)));
  for y in 0..=y_max {
    for x in 0..=x_max {
      if dots.contains(&(x, y)) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!()
  }
  (first_left, 0)
}
