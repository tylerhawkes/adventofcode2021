pub fn compute(input: &str) -> (usize, usize) {
  let positions = input
    .split(',')
    .map(|s| s.parse::<u16>().unwrap())
    .collect::<Vec<_>>();
  let (min, max) = positions
    .iter()
    .copied()
    .fold((u16::MAX, u16::MIN), |l, r| (l.0.min(r), l.1.max(r)));
  let mut min_cost_1 = usize::MAX;
  for i in min..=max {
    let cost = positions
      .iter()
      .copied()
      .map(|x| x.max(i) as usize - x.min(i) as usize)
      .sum::<usize>();
    min_cost_1 = cost.min(min_cost_1);
  }
  let mut min_cost_2 = usize::MAX;
  for i in min..=max {
    let cost = positions
      .iter()
      .copied()
      .map(|x| {
        let diff = x.max(i) as usize - x.min(i) as usize;
        (diff * diff + diff) / 2
      })
      .sum::<usize>();
    min_cost_2 = cost.min(min_cost_2);
  }
  (min_cost_1, min_cost_2)
}
