use std::collections::VecDeque;

pub fn compute(input: &str) -> (usize, usize) {
  let mut fish_counts = vec![0; 9].into_iter().collect::<VecDeque<usize>>();
  input
    .split(",")
    .for_each(|s| fish_counts[s.parse::<usize>().unwrap()] += 1);
  let mut part_one = 0;
  for i in 0..256 {
    if i == 80 {
      part_one = fish_counts.iter().copied().sum();
    }
    let zero = fish_counts.pop_front().unwrap();
    fish_counts[6] += zero;
    fish_counts.push_back(zero);
  }

  (part_one, fish_counts.iter().copied().sum())
}
