
pub fn compute(input: &str) -> (usize, usize) {
  let ints = input.lines().map(|s|s.parse::<u64>().unwrap()).collect::<Vec<_>>();
  let part1 = ints.windows(2).filter(|x|x[1] > x[0]).count();
  let part2 = ints.windows(4).filter(|x|x[3] > x[0]).count();
  (part1, part2)
}