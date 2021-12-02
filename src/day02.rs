pub fn compute(input: &str) -> (usize, usize) {
  let mut horizontal = 0;
  let mut depth = 0;
  input.lines().for_each(| l|{
    let mut split = l.split(' ');
    let command = split.next().unwrap();
    let units = split.next().unwrap().parse::<usize>().unwrap();
    match command {
      "forward" => horizontal += units,
      "down" => depth += units,
      "up" => depth -= units,
      _ => unreachable!(),
    }
  });
  (horizontal * depth, part2(input))
}

fn part2(input: &str) -> usize {
  let mut horizontal = 0;
  let mut depth = 0;
  let mut aim = 0;
  input.lines().for_each(| l|{
    let mut split = l.split(' ');
    let command = split.next().unwrap();
    let units = split.next().unwrap().parse::<usize>().unwrap();
    match command {
      "forward" => {
        horizontal += units;
        depth += aim * units;
      },
      "down" => aim += units,
      "up" => aim -= units,
      _ => unreachable!(),
    }
  });
  horizontal * depth
}
